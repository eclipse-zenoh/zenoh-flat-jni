# Publishing zenoh-flat-jni

This document describes how `zenoh-flat-jni` is built, verified, and published
so that `zenoh-java` and `zenoh-kotlin` can depend on it as an ordinary Maven
artifact.

It describes the pipeline as it exists in this repository. Where something is
not yet implemented or not yet exercised, it is listed under
[Known gaps](#known-gaps) rather than described as if it worked.

If you just need to run a release, go to [Running a release](#running-a-release).
If Maven Central is unfamiliar, read
[Background](#background-if-you-do-not-work-in-the-jvm-ecosystem) first.

## Contents

- [Background, if you do not work in the JVM ecosystem](#background-if-you-do-not-work-in-the-jvm-ecosystem)
  - [Coordinates, and why they are permanent](#coordinates-and-why-they-are-permanent)
  - [The file set Central requires](#the-file-set-central-requires)
  - [Why this library is not an ordinary JAR](#why-this-library-is-not-an-ordinary-jar)
- [Running a release](#running-a-release)
  - [Before the first run](#before-the-first-run)
  - [Rehearsal (dry run)](#rehearsal-dry-run)
  - [The real release](#the-real-release)
  - [After a release](#after-a-release)
  - [Cleaning up after a rehearsal](#cleaning-up-after-a-rehearsal)
- [What gets published](#what-gets-published)
  - [Desktop JVM](#desktop-jvm)
  - [Android](#android)
  - [Maven Central contents](#maven-central-contents)
- [Release relationship](#release-relationship)
- [How the pipeline works](#how-the-pipeline-works)
  - [1. `tag` — branch, bump, tag](#1-tag--branch-bump-tag)
  - [2. `publish` — both artifacts, one staging repository](#2-publish--both-artifacts-one-staging-repository)
  - [3. `publish-github`](#3-publish-github)
- [How a publication reaches Maven Central](#how-a-publication-reaches-maven-central)
- [Reproducibility](#reproducibility)
- [Building and inspecting artifacts locally](#building-and-inspecting-artifacts-locally)
- [Required secrets](#required-secrets)
- [Downstream release requirements](#downstream-release-requirements)
- [Known gaps](#known-gaps)
- [Release checklist](#release-checklist)

## Background, if you do not work in the JVM ecosystem

Skip this section if Maven Central is familiar.

### Coordinates, and why they are permanent

**Maven Central** is the JVM world's crates.io: one public repository that
essentially every Java/Kotlin build downloads dependencies from. A library is
identified by three fields, its *coordinates*:

```text
org.eclipse.zenoh : zenoh-flat-jni : 1.9.0
     groupId          artifactId     version
```

Two differences from crates.io shape everything below.

**Central serves compiled artifacts, not sources.** Cargo downloads a crate's
source and compiles it on your machine; Maven Central serves a **JAR** — a zip
of already-compiled `.class` files plus whatever else the author put inside.
Nothing is compiled at install time, so *we* must build, in advance, everything
a user could possibly need, for every platform they might be on.

**Releases are immutable.** Once `1.9.0` is published it can never be changed or
deleted; a mistake is fixed only by releasing `1.9.1`. That is why every check in
this pipeline runs *before the release step* — note that uploading is not yet
releasing, since artifacts first land in a private staging repository that can be
discarded. It is the release of that repository that has no undo.

### The file set Central requires

| File | What it is |
| --- | --- |
| `zenoh-flat-jni-1.9.0.jar` | the library itself |
| `...-sources.jar` | the source code, so IDEs can show it |
| `...-javadoc.jar` | API documentation, generated here by **Dokka** (Kotlin's doc tool) |
| `...pom` | metadata: coordinates, licence, developers, SCM URL, and the library's own dependencies |
| `...module` | Gradle's richer equivalent of the POM — *optional*, and only produced for the JVM publication |
| `.md5`, `.sha1`, … | checksums for each of the above |
| `.asc` | a **GPG signature** for each — Central rejects unsigned releases |

**Gradle** is the build tool (`./gradlew`). It compiles the Kotlin, assembles
the JAR, generates the POM, signs everything and uploads. `./gradlew` is the
*wrapper*: a small script plus `gradle-wrapper.jar` that fetches the exact
Gradle version this project expects, so every machine builds with the same one.

### Why this library is not an ordinary JAR

A normal Kotlin library is pure bytecode and runs anywhere the JVM does. This
one binds to Rust: the implementation is a native library —
`libzenoh_flat_jni.so` on Linux, `.dylib` on macOS, `.dll` on Windows — reached
through **JNI** (Java Native Interface, the JVM's FFI).

Native code is not portable, so the published JAR carries **six** of them, and
at startup the Kotlin code detects the current platform, extracts the matching
library from the JAR to a temp directory, and loads it. A release therefore has
to cross-compile Rust for six targets across three operating systems and collect
the results before Gradle can build a single JAR — which is why the pipeline is
a matrix of build jobs feeding one publish job.

Android is a second, separate artifact: an **AAR** (Android's library format — a
zip holding `classes.jar`, a manifest and `jni/<abi>/` native libraries) built
for four CPU ABIs.

## Running a release

Everything is driven from **Actions → Release → Run workflow** on `main`. There
is no tag to push and no local command to run: the workflow creates the release
branch, bumps the version, tags it, builds, verifies and publishes.

### Before the first run

- **Secrets are already in place.** `CENTRAL_SONATYPE_TOKEN_*` and `ORG_GPG_*`
  are organization-level secrets on `eclipse-zenoh`, inherited by this
  repository automatically. Nothing is stored here, and nothing needs
  configuring. See [Required secrets](#required-secrets).
- **Always supply `version`.** Without it, `create-release-branch` falls back to
  `git describe`, and no tag is reachable from `main`, so the run aborts on its
  first step. (`1.9.0-rc1` exists, but it sits on a dry-run branch.) The
  fallback applies whenever `version` is omitted — it has nothing to do with
  `live-run`.

### Rehearsal (dry run)

| Field | Value |
| --- | --- |
| Use workflow from | `main` |
| `live-run` | **unchecked** |
| `version` | `<fresh-rehearsal-version>` — provisional, and not one already used |
| `zenoh-version` | empty, unless the Zenoh dependency is being moved |
| `branch` | empty |
| `maven_publish` | checked — or uncheck for the very first run |

#### What the two switches actually control

They are independent, and neither is "do nothing". `live-run` decides *where* a
publication lands; `maven_publish` decides *whether* one happens at all.

| | `live-run` **unchecked** | `live-run` **checked** |
| --- | --- | --- |
| version | `<version>-SNAPSHOT` | `<version>` |
| repository | `central.sonatype.com/repository/maven-snapshots/` — **mutable** | staging, then the release repository — **immutable** |
| `closeAndReleaseSonatypeStagingRepository` | not run | run |
| coordinate guards (below) | skipped — gated on `snapshot == false` | run |
| branch and tag | `release/dry-run/<version>` | `release/<version>` |
| GitHub release | not created | created |
| reversible | yes — overwrite or ignore the snapshot | **no** |

The **coordinate guards** are two checks in the `publish` job. Before anything is
uploaded, each asks Maven Central whether the version being released already
exists — for `org.eclipse.zenoh:zenoh-flat-jni`, `…-jvm` and `…-android`:

```text
https://repo1.maven.org/maven2/org/eclipse/zenoh/<artifact>/<version>/<artifact>-<version>.pom
```

If either responds, the release stops. A published version can never be replaced
([Coordinates, and why they are permanent](#coordinates-and-why-they-are-permanent)),
so republishing cannot succeed — and discovering that part-way through an upload
is a confusing way to find out. Both are checked before either is published,
because an occupied *Android* coordinate found after the JVM artifact went public
would have burned a version number.

A rehearsal skips them because it does not publish under those coordinates at
all: a `-SNAPSHOT` version lives in a different, mutable repository.

So **`live-run` unchecked with `maven_publish` checked still performs a real
upload.** It authenticates to Sonatype with the real tokens, signs with the real
GPG key, and transfers the artifacts — into the snapshot repository, under a
`-SNAPSHOT` version. Nothing is staged, nothing is released, and
`org.eclipse.zenoh:zenoh-flat-jni:<version>` is untouched. Snapshots may be
overwritten freely, so a bad one costs nothing.

That is precisely why it is worth doing: **it is the only configuration that
exercises the credentials and the signing key.** With `maven_publish` unchecked
the Gradle publish step is skipped outright, so nothing reaches Sonatype and
nothing is learned about whether `CENTRAL_SONATYPE_TOKEN_*` and `ORG_GPG_*`
work. Ranked by what each proves:

1. `live-run` off, `maven_publish` **off** — builds, artifact verification,
   consumer test. Nothing reaches Maven.
2. `live-run` off, `maven_publish` **on** — all of that, **plus** signing,
   credentials and a genuine upload. Reversible.
3. `live-run` **on** — all of that, then closes and releases. **Not reversible.**

**Do not give a rehearsal the version you intend to release.**
`bump-and-tag.bash` tags and pushes whatever version it is handed, dry runs
included — deliberately, because dry-run branches and tags are throwaway names.
Passing `1.9.0` would leave a `1.9.0` tag pointing at a dry-run commit. A stray
Git tag can be deleted; a Maven Central release cannot.

#### Budget for two rehearsals

Each with its own fresh version — level 1 then level 2 above. Splitting them
separates "does it build and verify" from "do the credentials work", which makes
a first failure far easier to read.

Note that even level 1 is not inert: it pushes the release branch and tag, and
uploads the built libraries as Actions artifacts. What it does not do is reach
Maven.

What to look at in the run:

- **`Show the downloaded layout`** — the collected natives. Artifact layout is
  the likeliest thing to be wrong the first time a matrix runs.
- **`Smoke-test the candidate as an external consumer`** — expect
  `zenoh-flat-jni smoke test OK on <platform>` on Linux, macOS and Windows.
- **The job summary** — per-target SHA-256s and the measured glibc floor of each
  Linux artifact.

The "coordinates are still free on Maven Central" line does *not* appear in a
rehearsal — the coordinate guards described above are gated on
`snapshot == false`, and every rehearsal is a snapshot. Its absence is correct,
not a fault.

### The real release

| Field | Value |
| --- | --- |
| Use workflow from | `main` |
| `live-run` | **checked** |
| `version` | the release number, e.g. `1.9.0` |
| `zenoh-version` | the Zenoh release, if this release follows one |
| `maven_publish` | checked |

Supplying `zenoh-version` additionally re-points every `zenoh.*` dependency at
`release/<zenoh-version>` and refreshes `Cargo.lock`; leaving it empty releases
against whatever the manifest already declares.

The irreversible moment is the single `closeAndReleaseSonatypeStagingRepository`
at the end of the `publish` job. Both publications are uploaded by one Gradle
invocation, so they share one staging repository and are released by that one
call: either both artifacts become public or neither does.

This is why the two publish workflows were merged into one. Run as separate
workflows — in either order — whichever released first would already be public
and unwithdrawable when the second failed, and the second workflow's coordinate
guard would not even have run yet.

Everything before it — cross-build, verification, consumer test,
staging upload, Central's own validation — is reversible, but *not*
self-cleaning: a staging repository that fails validation is **left in place**.
The Nexus plugin throws when the repository does not reach the expected state
and never issues a drop, so the operator has to inspect and drop it in the
Central Portal. Before that point the job also refuses to continue if the
coordinates already exist on Central, since republishing cannot succeed.

### After a release

- Confirm **both** coordinates resolve — releasing them together is the whole
  point of the single staging repository, so verifying one proves half of it:

  ```text
  https://repo1.maven.org/maven2/org/eclipse/zenoh/zenoh-flat-jni/<version>/
  https://repo1.maven.org/maven2/org/eclipse/zenoh/zenoh-flat-jni-android/<version>/
  ```

  Central can take some minutes to index a new release.
- Only then release `zenoh-java` and `zenoh-kotlin` against it, per
  [Release relationship](#release-relationship) and
  [Downstream release requirements](#downstream-release-requirements).

### Cleaning up after a rehearsal

A rehearsal leaves a `release/dry-run/<version>` branch, a tag of the same
version, and a snapshot. The branches are pruned automatically — the last few
are kept — and the snapshot is mutable, so both can be left alone. Delete the
tag and branch if you would rather not keep them:

```bash
git push origin --delete release/dry-run/<version>
git push origin --delete <version>
```

**Always use a version no rehearsal has used before.** `bump-and-tag.bash` runs
`git tag --force` and force-pushes, so re-running a rehearsal under an existing
version silently moves that tag to a new commit — including a tag someone is
relying on as a record. `1.9.0-rc1` is already taken by the first rehearsal;
`1.9.0-rc2` is the next free one.

## What gets published

### Desktop JVM

```text
org.eclipse.zenoh:zenoh-flat-jni:<version>          root — Gradle module metadata
org.eclipse.zenoh:zenoh-flat-jni-jvm:<version>      the JVM artifact
org.eclipse.zenoh:zenoh-flat-jni-android:<version>  the Android artifact
```

This is a **Kotlin Multiplatform** library, so a consumer declares the *root*
coordinate once and Gradle resolves the variant matching its target. That is what
makes it structurally impossible to build an Android app against the desktop
libraries — and it lets a consumer publish its own JVM and Android artifacts from
a single Gradle invocation, which is what allows an atomic release downstream.

A universal JVM JAR: the Kotlin/JVM classes plus one native library per
supported desktop target. `NativeLibrary.kt` resolves them from this layout,
which the release build reproduces exactly:

```text
x86_64-unknown-linux-gnu/x86_64-unknown-linux-gnu.zip
aarch64-unknown-linux-gnu/aarch64-unknown-linux-gnu.zip
x86_64-apple-darwin/x86_64-apple-darwin.zip
aarch64-apple-darwin/aarch64-apple-darwin.zip
x86_64-pc-windows-msvc/x86_64-pc-windows-msvc.zip
aarch64-pc-windows-msvc/aarch64-pc-windows-msvc.zip
```

Each ZIP holds exactly the release native library for that target:

- Linux: `libzenoh_flat_jni.so`
- macOS: `libzenoh_flat_jni.dylib`
- Windows: `zenoh_flat_jni.dll`

**Linux runtime requirement: glibc 2.28 or newer; currently measured at 2.18.** The two Linux targets are
built with `cross`, inside a container whose glibc is far older than the CI
runner's, because a native `ubuntu-latest` (24.04) build requires `GLIBC_2.39`
and would refuse to load on Ubuntu 22.04, Debian 12 or RHEL 9. Both floors were
measured with `objdump -T`:

```text
built natively on ubuntu-24.04   GLIBC_2.39
built with cross (both targets)  GLIBC_2.18
```

Those are two different numbers on purpose. **2.28 is the compatibility
contract** — what a consumer may rely on. **2.18 is what today's `cross` images
happen to produce**, and it is reported in every release's job summary.

`Check the glibc requirement` re-measures on every release and **fails the build**
above `SUPPORTED_GLIBC` (2.28). So a floor that rises *within* that range — say a
future image producing 2.25 — is reported but not rejected: it stays inside the
contract, and consumers on 2.28 or newer are unaffected. Only a rise past 2.28
stops the release. If you need the stricter promise, set `SUPPORTED_GLIBC` to
`2.18` and every *upward drift in the measured glibc requirement* becomes a
release failure to be reviewed by hand. Note that is all it catches: an image can
change its digest, compiler and sysroot contents, or the output bytes, while the
highest glibc symbol the library needs stays where it was.

That check is what bounds glibc compatibility, because the toolchain pin does
not. `CROSS_VERSION` pins the `cross` *executable*; the image it selects is
`ghcr.io/cross-rs/<target>:<version>`, a mutable tag rather than a digest, so the
sysroot can move underneath a fixed tool version. Rather than pin two image
digests and keep them current, the release enforces the one compatibility
property it is prepared to promise. It bounds *that* property and nothing else —
a changed image can still alter the artifact in other ways.

The `desktopTargets` map in `build.gradle.kts` drives `verifyDesktopArtifact`,
so a target missing from a build fails the release rather than shipping a JAR
that silently lacks it. It does **not** drive the build: the same six targets are
listed again in the matrix in `publish.yml`, and the two must be kept in
step by hand. Adding a target means editing both — the verifier is what makes
forgetting the second one loud instead of silent.

A developer build produces a different JAR — the host library alone, at the JAR
root (`NativeLibrary`'s second loading strategy). The two layouts are mutually
exclusive by construction: when `jni-libs/` is present, the host library is not
bundled and no Cargo build is wired into `jar` at all.

### Android

```text
org.eclipse.zenoh:zenoh-flat-jni-android:<version>
```

An AAR with the Android NDK libraries in the standard layout:

```text
jni/armeabi-v7a/libzenoh_flat_jni.so
jni/arm64-v8a/libzenoh_flat_jni.so
jni/x86/libzenoh_flat_jni.so
jni/x86_64/libzenoh_flat_jni.so
```

The AAR is assembled by a plain `Zip` task, not by the Android Gradle Plugin.
This module has no Android resources, no manifest entries beyond `minSdk`, and
no Android-only code, so its AAR is `classes.jar` plus `jni/<abi>/` — applying
AGP would mean installing an Android SDK in CI to zip four `.so` files. The
`androidAar` task synthesises the mandatory `AndroidManifest.xml` and empty
`R.txt`, and `androidClassesJar` deliberately packs `output.classesDirs` rather
than the desktop `jar`, so the AAR does not carry the desktop native ZIPs.

Switch to `com.android.library` if this module ever grows real Android
resources or a non-trivial manifest.

Downstream Android publications must resolve this AAR, not the desktop JAR.

### Maven Central contents

Both publications carry:

- The primary JAR or AAR.
- A sources JAR (`sourcesJar`: Kotlin sources, the Rust `src/`, `build.rs`,
  `Cargo.toml`).
- A Dokka-generated Javadoc JAR (`javadocJar`).
- A POM with name, description, URL, license, developers, and SCM information.
  The Android POM is built from `artifact()` entries, so its single runtime
  dependency on `kotlin-stdlib` is written explicitly.
- A PGP signature per artifact and POM (signing is required only when
  `-PremotePublication=true`).
- Checksums, generated during publication.

The tag, `version.txt`, `Cargo.toml`, and the Maven version must agree.
`version.txt` is the single source of truth: `bump-and-tag.bash` writes it and
propagates it to `Cargo.toml`, and the `publish` job re-checks that the two still
agree before publishing. `gradle.properties` no longer carries a second
copy.

## Release relationship

`zenoh-flat-jni` owns the generated Kotlin/JNI boundary and the native
libraries. `zenoh-java` and `zenoh-kotlin` are downstream wrappers and must not
build or package their own copies of the JNI library.

The release order is:

```text
prebindgen from crates.io, zenoh-flat and Zenoh pinned by Cargo.lock
                         |
                         v
             publish zenoh-flat-jni
                         |
                         v
        verify it resolves from Maven Central
                    /           \
                   v             v
          release zenoh-java  release zenoh-kotlin
```

Maven Central releases are immutable. A downstream release must therefore never
depend on an unpublished `zenoh-flat-jni` version or assume that an already
published JNI artifact can be replaced later.

## How the pipeline works

Releases are driven by
[`.github/workflows/release.yml`](.github/workflows/release.yml), the same shape
zenoh-java and zenoh-kotlin use. It is started manually
(`workflow_dispatch`). `release.yml` itself defines three jobs — tag, publish,
publish-github — and the `publish` job is a call into `publish.yml`, which
contributes four of its own.

### 1. `tag` — branch, bump, tag

`eclipse-zenoh/ci/create-release-branch` creates the release branch, then
[`ci/scripts/bump-and-tag.bash`](ci/scripts/bump-and-tag.bash) writes the
version into `version.txt` and `Cargo.toml`, re-points every `zenoh.*`
dependency at the Zenoh release branch, refreshes `Cargo.lock`, commits and
tags.

With `live-run: false` the release is not skipped but *redirected*: the branch
becomes `release/dry-run/<version>`, and the version comes from `git describe`
if — and only if — no `version` input was supplied.
That is the dry run.

### 2. `publish` — both artifacts, one staging repository

[`publish.yml`](.github/workflows/publish.yml), called as a reusable workflow.
Everything is built and checked before anything is uploaded, and both
publications are released together:

- **`desktop-natives`** cross-compiles the six declared targets with `--locked`
  and packages each as `<target>/<target>.zip` with its SHA-256 in the job
  summary. The toolchain is not named in the workflow — `rust-toolchain.toml`
  supplies it, so the release uses the compiler CI uses. On the host target it
  also fails if `src/generated_bindings.rs`, `kotlin/generated` or
  `kotlin/REPORT.md` differ from what is committed: a release must not ship
  generated sources nobody reviewed.

  Both Linux targets are built with `cross`, at a pinned `CROSS_VERSION`, and
  each records its measured glibc floor in the job summary.

- **`android-natives`** builds the four ABIs by running `./gradlew
  buildAndroidLibs` — the same task the README gives developers, so the two
  cannot drift — with the pinned cargo-ndk and NDK. `cargo ndk -o` writes
  exactly the AAR's `jni/<abi>/` layout, so nothing is repackaged.

  The `publish` job passes `-PprebuiltAndroidLibs=true`, which is the *only*
  way to skip the cross-compilation: there the libraries arrive as downloaded
  build artifacts and the runner has no NDK. Everywhere else the task runs
  cargo-ndk unconditionally and lets Cargo decide what is up to date, since a
  directory holding four correctly-named files says nothing about whether they
  are current — or even the right architecture.

- **`consumer-test`** assembles the JVM publication, publishes it to an isolated
  file repository under `build/dry-run-repository`, and runs
  [`ci/consumer-smoke-test`](ci/consumer-smoke-test) against it as an external
  Gradle project — no path dependency, no composite build. Its repository
  declarations use content filters (`includeGroup` on the candidate repository,
  `excludeGroup` on Maven Central), so resolution cannot silently fall back to a
  previously released artifact with the same coordinates: if it builds, the
  candidate is what it resolved. `mavenLocal()` is deliberately not used, since
  it can serve leftovers from earlier builds.

  The smoke test creates a key expression, round-trips it across JNI and closes
  the handle. Key expressions need no network, ports or discovery, so a runner
  cannot make it flaky, and it still covers native-library extraction from the
  JAR, a JNI call in both directions, and a handle's create/use/close cycle.

  It runs on Linux, macOS and Windows.

- **`publish`** checks that `version.txt` and `Cargo.toml` agree, then checks
  **both** coordinates — `zenoh-flat-jni` and `zenoh-flat-jni-android` — against
  Maven Central and refuses to proceed if either is taken. Doing both before
  either is published is the point: an occupied Android coordinate discovered
  after the JVM release would have burned a version number.

  It then assembles and verifies both artifacts, unconditionally, so a
  `maven_publish: false` rehearsal still exercises every check.
  `verifyDesktopArtifact` fails on a missing target ZIP, a ZIP whose contents
  are not exactly the one expected library, or a stray native at the JAR root
  that would shadow the per-target resources; `verifyAndroidArtifact` checks the
  manifest, `classes.jar`, `R.txt` and all four ABI libraries.

  Only then does it publish — see
  [How a publication reaches Maven Central](#how-a-publication-reaches-maven-central).

### 3. `publish-github`

`eclipse-zenoh/ci/publish-crates-github` creates the GitHub release, after both
Maven publications have succeeded.

## How a publication reaches Maven Central

Publishing is one Gradle command, covering both publications:

```bash
./gradlew publishMavenPublicationToSonatypeRepository \
          publishAndroidPublicationToSonatypeRepository \
          closeAndReleaseSonatypeStagingRepository \
          -PremotePublication=true -Prelease=true -PprebuiltAndroidLibs=true
```

`-PprebuiltAndroidLibs=true` matters here: without it `androidAar` would schedule
`buildAndroidLibs`, which on the publishing runner fails for want of an NDK — and
on a machine that has one would rebuild the libraries instead of publishing the
verified ones that were downloaded.

Three steps hide in there, run by
`io.github.gradle-nexus.publish-plugin`:

1. **Upload to a staging repository.** Not public — a private holding area on
   Sonatype's Central Portal.
2. **Close it.** Central now validates: are all signatures present and valid, do
   the checksums match, does the POM carry a name, description, licence,
   developer and SCM URL? Nothing is public yet, so a failure here publishes
   nothing — but it is not self-cleaning either. The plugin throws
   `RepositoryTransitionException` when the repository does not reach the
   expected state and never issues a drop, so the staging repository is left for
   the operator to inspect and drop in the Central Portal.
3. **Release it.** *This is the irreversible step.* The artifacts go public and
   can never be changed.

Every gate described above exists to run before step 3, and step 3 happens
**once** per release: both publications are uploaded into the same staging
repository by one Gradle invocation, so the single close-and-release either
publishes both artifacts or neither.

Two mechanical details are worth knowing, because both previously made a release
impossible in this repository:

- **Gradle properties are passed with `-P`, not `-D`.** The build reads
  `remotePublication` and `release` through `project.findProperty`, but the old
  workflow passed them as `-D` system properties. They were therefore always
  absent, so a tag build quietly selected the *debug* library and published to
  the runner's own machine instead of Central.
- **Where the credentials come from.** Nothing is stored in this repository.
  `CENTRAL_SONATYPE_TOKEN_*` and `ORG_GPG_*` are organization-level secrets on
  `eclipse-zenoh`, inherited automatically — which is why using the same secret
  names as zenoh-java matters. A reusable workflow does *not* receive them
  unless the caller says `secrets: inherit`, which `release.yml` does.

```text
nexusUrl               https://ossrh-staging-api.central.sonatype.com/service/local/
snapshotRepositoryUrl  https://central.sonatype.com/repository/maven-snapshots/
```

The legacy `s01.oss.sonatype.org` endpoint is retired and its OSSRH credentials
no longer work; the credentials are Central Portal tokens.

**Snapshots are the exception to immutability.** For a dry run the workflows
pass `-PSNAPSHOT` and omit `closeAndReleaseSonatypeStagingRepository`: the
version becomes `<version>-SNAPSHOT` and goes to the snapshot repository, which
is *mutable*, may be overwritten freely, and skips staging validation entirely.
Useful for repeated integration testing — but a snapshot passing proves nothing
about whether a real release would survive Central's validation.

## Reproducibility

Every archive task sets `isPreserveFileTimestamps = false` and
`isReproducibleFileOrder = true`, so the JAR's *own* entries are normalized.

That does **not** make the JAR byte-identical between builds. The native
libraries are wrapped with plain `zip -j` in the workflow, which records each
file's modification time, and those ZIP bytes are embedded in the JAR unchanged
— so two builds of identical native code still produce different JAR hashes.
Normalizing the nested archives too (a fixed mtime before zipping) would close
the gap, but nothing currently depends on it: the post-release hash comparison
that once justified the claim is no longer part of the pipeline.

These inputs are pinned:

| Input | How it is fixed |
| --- | --- |
| `zenoh`, `zenoh-ext`, `zenoh-flat` | `version` + `git` + `branch` in `Cargo.toml`; the release bump re-points `branch` at `release/X.Y.Z` |
| every transitive crate | `Cargo.lock`, committed and kept byte-aligned with Zenoh's by the shared lockfile-sync bot |
| the Rust compiler | `rust-toolchain.toml` |
| `cargo-ndk`, the NDK | pinned by exact version in `publish.yml`; `cargo install --locked` pins the dependencies of the *selected* release, not which release is selected, so each needs its own version pin |
| `cross` | the *executable*, by exact version — see below for its image |

Every release build runs `--locked`, so a lockfile that does not match the
manifest fails the build instead of silently resolving something else.

These do **not** pin, and a release is not bit-for-bit reproducible across time
because of them:

| Floating input | Consequence |
| --- | --- |
| the `cross` images (`ghcr.io/cross-rs/<target>:<version>`) | a mutable tag, not a digest, so the Linux sysroot can change under a fixed tool version. The glibc check bounds only the *glibc-floor effect* of such a change |
| every runner image (`ubuntu-latest`, `macos-latest`, `windows-latest`) | host toolchains, SDKs and linkers move with GitHub's images — `ubuntu-latest` runs the Linux, Android, tag and publication jobs, so it reaches the Android and Kotlin artifacts too |
| the JDK (`java-version: 11`) | a major version, not a patch release, so the compiler that builds the Kotlin classes can change |
| the workflow actions (`@v4`, `@v1`) | major-version tags, so action behaviour can change |
| `eclipse-zenoh/ci/create-release-branch@main`, `…/publish-crates-github@main` | a moving branch, not even a tag: the tagging and GitHub-release steps track whatever `main` holds at run time |

Pinning all of these would mean carrying image digests and action SHAs and
keeping them current. That maintenance has not been taken on; what the release
does guarantee is the dependency graph (`Cargo.lock`), the compiler
(`rust-toolchain.toml`) and the glibc bound.

## Building and inspecting artifacts locally

Publishing locally goes to the same isolated repository the CI consumer test
uses. The commands below are written for an **x86-64 Linux host** — on another
platform, substitute that host's target triple and library name from the table
in [Desktop JVM](#desktop-jvm), both in the real archive and in the placeholder
list.

One thing to know first: **`verifyDesktopArtifact` demands all six targets**,
and a developer machine can only build its own. Publishing with just
the host library staged therefore fails by design —

```text
missing native resource `aarch64-apple-darwin/aarch64-apple-darwin.zip`
missing native resource `x86_64-pc-windows-msvc/x86_64-pc-windows-msvc.zip`
…
```

— which is the verifier doing its job. To exercise the rest of the path, stage
the host library for real and stand in placeholders for the other five. The
placeholder names must match what the verifier expects per target, since it
checks each archive holds exactly its one library:

```bash
cargo build --release --locked

# the host target, for real
mkdir -p jni-libs/x86_64-unknown-linux-gnu
(cd target/release && zip -j ../../jni-libs/x86_64-unknown-linux-gnu/x86_64-unknown-linux-gnu.zip libzenoh_flat_jni.so)

# the other five, so the verifier is satisfied and the JAR is assembled
stub() { mkdir -p "jni-libs/$1"; : > "$2"; zip -jq "jni-libs/$1/$1.zip" "$2"; rm -f "$2"; }
stub aarch64-unknown-linux-gnu libzenoh_flat_jni.so
stub x86_64-apple-darwin       libzenoh_flat_jni.dylib
stub aarch64-apple-darwin      libzenoh_flat_jni.dylib
stub x86_64-pc-windows-msvc    zenoh_flat_jni.dll
stub aarch64-pc-windows-msvc   zenoh_flat_jni.dll

./gradlew publishMavenPublicationToDryRunRepository -Prelease=true
find build/dry-run-repository -type f -print

# the wrapper, not a system `gradle`: the consumer needs the Gradle version this
# project pins
./gradlew --project-dir ci/consumer-smoke-test run --refresh-dependencies \
  -PcandidateRepository="file://$PWD/build/dry-run-repository" \
  -PcandidateVersion="$(cat version.txt)"
```

The smoke test loads the *host* library out of the JAR, so the placeholders never
get loaded — it exercises the real extraction and JNI path on this machine while
the other five entries only satisfy the layout check. Delete `jni-libs/`
afterwards; it is gitignored, but leaving it changes what a subsequent local
build produces.

Inspect the archives and signatures directly:

```bash
unzip -l build/libs/zenoh-flat-jni-*.jar
unzip -l build/distributions/zenoh-flat-jni-android-*.aar
gpg --verify path/to/artifact.asc path/to/artifact
```

Two properties are easy to confuse:

- **`-PremotePublication=true`** switches on GPG signing and makes the build
  fail fast unless `jni-libs/` or `android-libs/` is populated, so a *remote*
  publication cannot silently ship the publishing runner's own host library. It
  is omitted from the recipe above only because a local dry run has no signing
  keys.
- **`jni-libs/` existing** is what selects the multi-platform layout. Populate
  it, as the recipe does, and the dry run produces the same multi-platform JAR a
  release would — without it the build falls back to the developer layout, with
  the host library at the JAR root, which is *not* the published artifact.
- **`-PprebuiltAndroidLibs=true`** tells `buildAndroidLibs` that `android-libs/`
  is already the artifact to package, rather than something to rebuild. Pass it
  whenever the libraries came from somewhere other than this machine's cargo-ndk
  — the release does, because they arrive as downloaded build artifacts on a
  runner with no NDK. Leave it off and Cargo decides what is stale, which is what
  you want while developing.

## Required secrets

| Secret | Use |
| --- | --- |
| `CENTRAL_SONATYPE_TOKEN_USERNAME` | Central Portal user token |
| `CENTRAL_SONATYPE_TOKEN_PASSWORD` | Central Portal user token |
| `ORG_GPG_SUBKEY_ID` | signing |
| `ORG_GPG_PRIVATE_KEY` | signing |
| `ORG_GPG_PASSPHRASE` | signing |
| `BOT_TOKEN_WORKFLOW` | creating and pushing the release branch and tag, and creating the GitHub release |

## Downstream release requirements

Before releasing `zenoh-java` or `zenoh-kotlin`:

- The selected `zenoh-flat-jni` version must already resolve from Maven Central.
- The dependency version must live in one release-controlled property or version
  catalog, not be duplicated as a string.
- Local composite substitution (`includeBuild("../zenoh-flat-jni")`) must be
  optional and disabled in release CI.
- `zenoh-java`'s obsolete workflow that builds the removed `zenoh-jni/Cargo.toml`
  must be deleted.
- JVM publications depend on the desktop JVM artifact; Android publications
  depend on the Android artifact.
- Published downstream POM and Gradle metadata must contain the intended
  `zenoh-flat-jni` dependency.
- Downstream tests must pass using only released Maven artifacts.

## Known gaps

The workflow is implemented; what follows has not been exercised. In short: the
full release matrix, the Android runtime path, the Central upload, and the
downstream release suites.

- **The Android artifact has no runtime test.** The AAR's contents are verified
  by archive inspection; nothing loads it. This needs an Android consumer app
  and an emulator job. Until then the Android publication is unproven at
  runtime, and the `CARGO_NDK_VERSION` pin is a current-stable choice rather
  than a validated one.
- **`aarch64-pc-windows-msvc` is cross-compiled and archive-inspected, never
  loaded.** Nothing technical prevents it any more, and both halves were checked
  by running jobs rather than reading documentation:

  - a GitHub-hosted Windows ARM64 runner is available to this repository
    (`windows-11-arm`);
  - **Java 11 is available there** — `distribution: microsoft` installs
    `microsoft-jdk-11.0.31-windows-aarch64` natively. Only Temurin is missing
    (`Could not find satisfied version for SemVer '11'`), which is a
    distribution gap, not a platform one.

  So an ARM64 consumer job could run on the project's Java 11 toolchain. It is
  simply not written yet; adding it would close this gap.
- **The cross-build matrix has not yet completed in CI.** Two rehearsals have
  run. The first
  ([31312017566](https://github.com/eclipse-zenoh/zenoh-flat-jni/actions/runs/31312017566))
  failed every job on a stale lockfile, fixed by #28. The second
  ([31320816585](https://github.com/eclipse-zenoh/zenoh-flat-jni/actions/runs/31320816585))
  got much further: **all six desktop targets compiled**, including
  `aarch64-apple-darwin` and `aarch64-pc-windows-msvc`, and all four Android
  ABIs compiled. It still failed — the Windows jobs on a missing checksum tool
  after packaging had succeeded, and Android on the crate type — both fixed
  here.

  What remains unexercised is the matrix *completing*, and every produced
  library except the host one being **loaded** rather than compiled and
  inspected. `consumer-test`, the publication and the Central upload have never
  run.
- **The Central upload has never run.** A `maven_publish: false` rehearsal does
  not exercise signing or the Central credentials; only a rehearsal with
  publication enabled does.
- **Downstream release suites have not been run against a Maven artifact** with
  composite substitution disabled; those changes belong in `zenoh-java` and
  `zenoh-kotlin`.

## Release checklist

Most of this is enforced by the pipeline; the list is what a release owner
should confirm rather than assume.

- [ ] `Cargo.lock` is committed and current for the manifest — the version bump
      refreshes it, but a manifest edit landed by hand may not have.
- [ ] `version.txt` and `Cargo.toml` agree (the `publish` job re-checks).
- [ ] **Both** rehearsals completed under fresh versions that are *not* the one
      being released: one with `maven_publish: false`, and one with publication
      enabled — only the second exercises signing and the Central credentials.
- [ ] The version being released is free on Maven Central (the pipeline refuses
      otherwise).
- [ ] Generated sources are clean; Rust and Kotlin tests pass.
- [ ] All declared desktop targets are in the JAR and all ABIs in the AAR.
- [ ] Sources, Javadoc, POM, signatures, and checksums pass inspection.
- [ ] The external consumer test passed on every runner platform.
- [ ] Central staging validation passed.
- [ ] The released coordinates resolve from Maven Central.
