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

import io.zenoh.jni.config.ZenohId
import io.zenoh.jni.config.zidString
import kotlin.random.Random
import kotlin.test.Test
import kotlin.test.assertEquals

/**
 * Self-verification of the pure-Kotlin [zidString] formatter (the Zenoh id
 * string form, no JNI) against the native `ZenohId.toStr`, over edge byte
 * patterns and a seeded-random corpus. Both operate on the same generated
 * [ZenohId] data class.
 */
class ZenohIdCorrespondenceTest {

    private val boom = JniErrorHandler<Nothing> { je ->
        throw AssertionError("unexpected native zid binding error: $je")
    }

    private val domainBoom = ErrorHandler<Nothing> { ze ->
        throw AssertionError("unexpected native zid error: $ze")
    }

    private fun assertCorresponds(bytes: ByteArray) {
        val zid = ZenohId(bytes)
        assertEquals(
            zid.toStr(boom, domainBoom),
            zid.zidString(),
            "zid render mismatch for ${bytes.joinToString(",")}",
        )
    }

    @Test
    fun rendersLikeNative() {
        // No all-zero id: those bytes are not an identifier, so the native
        // renderer rejects them rather than producing a string to compare.
        val edges = listOf(
            ByteArray(16).also { it[0] = 1 }, // smallest nonzero (LE low byte)
            ByteArray(16).also { it[15] = 1 }, // highest byte only
            ByteArray(16).also { it[0] = 0x0F }, // sub-0x10 low byte
            ByteArray(16) { 0xFF.toByte() }, // all ones
            ByteArray(16) { i -> i.toByte() }, // ascending with leading zero byte
        )
        edges.forEach(::assertCorresponds)

        val rng = Random(20260716)
        repeat(32) { assertCorresponds(rng.nextBytes(16)) }
    }
}
