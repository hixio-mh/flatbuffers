// automatically generated by the FlatBuffers compiler, do not modify

package MyGame.Example2

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

@Suppress("unused")
@ExperimentalUnsignedTypes
class Monster : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : Monster {
        __init(_i, _bb)
        return this
    }
    companion object {
        fun validateVersion() = Constants.FLATBUFFERS_2_0_0()
        fun getRootAsMonster(_bb: ByteBuffer): Monster = getRootAsMonster(_bb, Monster())
        fun getRootAsMonster(_bb: ByteBuffer, obj: Monster): Monster {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        fun startMonster(builder: FlatBufferBuilder) = builder.startTable(0)
        fun endMonster(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}
