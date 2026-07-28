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
import io.zenoh.jni.pubsub.EntityGlobalId
import io.zenoh.jni.sample.SourceInfo
import io.zenoh.jni.time.Timestamp
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotEquals

/**
 * The byte-backed value types must compare by CONTENT.
 *
 * Their Rust counterparts derive `PartialEq`/`Eq`, so equal contents are the
 * same value. Kotlin arrays compare by identity, so this holds only because the
 * binding generates content-based operators — without them two identically
 * built values are unequal, which is the shape a consumer hits when it keys a
 * map on a peer id or compares a sample's timestamp.
 *
 * `hashCode` is asserted alongside because Kotlin's own `data class` codegen is
 * inconsistent: it special-cases arrays in `hashCode` but NOT in `equals`. A
 * broken value therefore hashes into the right bucket and is rejected there, so
 * `hashCode` agreement alone proves nothing — the `HashSet` checks are what pin
 * the actual contract.
 */
class ValueEqualityTest {

    @Test
    fun zenohIdComparesByContent() {
        val a = ZenohId(byteArrayOf(1, 2, 3))
        val b = ZenohId(byteArrayOf(1, 2, 3))
        assertEquals(a, b)
        assertEquals(a.hashCode(), b.hashCode())
        assertEquals(1, hashSetOf(a, b).size, "equal ids must de-duplicate")
        assertNotEquals(a, ZenohId(byteArrayOf(1, 2, 4)))
    }

    @Test
    fun timestampComparesByContent() {
        val a = Timestamp(1uL, byteArrayOf(1, 2, 3))
        val b = Timestamp(1uL, byteArrayOf(1, 2, 3))
        assertEquals(a, b)
        assertEquals(a.hashCode(), b.hashCode())
        assertEquals(1, hashSetOf(a, b).size, "equal timestamps must de-duplicate")
        // Both components must participate.
        assertNotEquals(a, Timestamp(2uL, byteArrayOf(1, 2, 3)))
        assertNotEquals(a, Timestamp(1uL, byteArrayOf(1, 2, 4)))
    }

    @Test
    fun nestedValuesComparesByContent() {
        // `EntityGlobalId` carries a `ZenohId`, and `SourceInfo` carries an
        // `EntityGlobalId` — the nesting is where the defect propagated.
        val e1 = EntityGlobalId(ZenohId(byteArrayOf(1)), 7)
        val e2 = EntityGlobalId(ZenohId(byteArrayOf(1)), 7)
        assertEquals(e1, e2)
        assertEquals(1, hashSetOf(e1, e2).size)
        assertNotEquals(e1, EntityGlobalId(ZenohId(byteArrayOf(2)), 7))
        assertNotEquals(e1, EntityGlobalId(ZenohId(byteArrayOf(1)), 8))

        val s1 = SourceInfo(EntityGlobalId(ZenohId(byteArrayOf(1)), 7), 3)
        val s2 = SourceInfo(EntityGlobalId(ZenohId(byteArrayOf(1)), 7), 3)
        assertEquals(s1, s2)
        assertEquals(1, hashSetOf(s1, s2).size)
        assertNotEquals(s1, SourceInfo(EntityGlobalId(ZenohId(byteArrayOf(1)), 7), 4))
    }

    @Test
    fun toStringRendersBytes() {
        // An array would otherwise render as `[B@1a2b3c`, which makes a logged
        // id or timestamp useless.
        assertEquals("ZenohId(bytes=[1, 2, 3])", ZenohId(byteArrayOf(1, 2, 3)).toString())
        assertEquals(
            "Timestamp(ntp64=1, id=[1, 2, 3])",
            Timestamp(1uL, byteArrayOf(1, 2, 3)).toString(),
        )
    }
}
