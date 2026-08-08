//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

package smoke

import io.zenoh.jni.keyexpr.KeyExpr

private fun fail(message: String?): Nothing =
    throw IllegalStateException(message ?: "unknown zenoh-flat-jni failure")

/**
 * The minimum that proves a published artifact is usable: loading the native
 * library out of the JAR resources, crossing the JNI boundary in both
 * directions, and taking a handle through its full create/use/close cycle.
 *
 * Key expressions are used rather than a session because they need no network,
 * no ports and no discovery — a CI runner cannot make them flaky.
 */
fun main() {
    val expr = "demo/example/**"
    val ke = KeyExpr.newTryFrom(expr, { fail(it) }, { fail(it) })
    try {
        val text = ke.toStr { fail(it) }
        check(text == expr) { "key expression round-tripped as `$text`, expected `$expr`" }
        check(ke.intersects("demo/example/smoke") { fail(it) }) {
            "`$expr` should intersect `demo/example/smoke`"
        }
        check(!ke.intersects("other/key") { fail(it) }) {
            "`$expr` should not intersect `other/key`"
        }
    } finally {
        ke.close()
    }
    println("zenoh-flat-jni smoke test OK on ${System.getProperty("os.name")} ${System.getProperty("os.arch")}")
}
