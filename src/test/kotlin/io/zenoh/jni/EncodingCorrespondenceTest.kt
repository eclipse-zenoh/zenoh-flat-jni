//
// Copyright (c) 2026 ZettaScale Technology
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

package io.zenoh.jni

import io.zenoh.jni.bytes.Encoding
import io.zenoh.jni.bytes.EncodingCodec
import kotlin.test.Test
import kotlin.test.assertEquals

/**
 * Self-verification of the pure-Kotlin [EncodingCodec] (encoding string ↔
 * `(id, schema)`, no JNI) against the native generated [Encoding] handle, over
 * the whole predefined id range and the parse/render edge shapes.
 */
class EncodingCorrespondenceTest {

    private val boom = JniErrorHandler<Nothing> { je ->
        throw AssertionError("unexpected native encoding error: $je")
    }

    /**
     * Native canonical string of `(id, schema)`.
     *
     * The native schema is raw bytes (it need not be valid UTF-8), while
     * [EncodingCodec] models the textual form this corpus is written in, so the
     * two are bridged by a UTF-8 transcode at the boundary — every schema
     * exercised here is text.
     */
    private fun nativeRender(id: Int, schema: String?): String {
        val h = Encoding.newFromId(id, schema?.encodeToByteArray(), boom)
        try {
            return h.toStr(boom)
        } finally {
            h.close()
        }
    }

    /** Native parse: a string into `(id, schema, canonical string)`. */
    private fun nativeParse(s: String): Triple<Int, String?, String> {
        val h = Encoding.newFromString(s, boom)
        try {
            return Triple(h.getId(boom), h.getSchema(boom)?.decodeToString(), h.toStr(boom))
        } finally {
            h.close()
        }
    }

    @Test
    fun renderMatchesNativeAcrossIdRange() {
        val ids = (0..64) + listOf(1000, 0xFFFE, 0xFFFF)
        for (id in ids) {
            for (schema in listOf(null, "utf-8")) {
                assertEquals(
                    nativeRender(id, schema),
                    EncodingCodec.render(id, schema),
                    "render mismatch for (id=$id, schema=$schema)",
                )
            }
        }
    }

    @Test
    fun parseMatchesNativeOnNamesAndEdges() {
        // Every predefined canonical name (obtained FROM the native table so the
        // corpus can't drift), plus the parse-rule edge shapes.
        val names = (0..52).map { nativeRender(it, null) }
        val edges = listOf(
            "", "text/plain", "text/plain;utf-8", "text/plain;", "my_custom_encoding",
            "custom;with;semicolons", ";leading_separator", "unknown_name;schema", "zenoh/bytes;s",
        )
        for (s in names + names.map { "$it;schema" } + edges) {
            val (nid, nschema, nstr) = nativeParse(s)
            val (id, schema) = EncodingCodec.parse(s)
            assertEquals(nid, id, "id mismatch for \"$s\"")
            assertEquals(nschema, schema, "schema mismatch for \"$s\"")
            assertEquals(nstr, EncodingCodec.render(id, schema), "render mismatch for \"$s\"")
        }
    }

    @Test
    fun withSchemaMatchesNative() {
        val bases = listOf(
            EncodingCodec.parse("text/plain"),
            EncodingCodec.parse("zenoh/bytes"),
            EncodingCodec.parse("my_custom_encoding"),
            EncodingCodec.parse("text/plain;old-schema"),
        )
        for ((id, schema) in bases) {
            val (pid, pschema) = EncodingCodec.withSchema(id, schema, "new-schema")
            val pure = EncodingCodec.render(pid, pschema)
            // The `e` param crosses on the selector's value arm as its
            // decomposed (id, schema) pair.
            val w = Encoding.newWithSchema(0, id, schema?.encodeToByteArray(), null, "new-schema", boom)
            val nativeStr = try {
                w.toStr(boom)
            } finally {
                w.close()
            }
            assertEquals(nativeStr, pure, "withSchema mismatch for (id=$id, schema=$schema)")
        }
    }
}
