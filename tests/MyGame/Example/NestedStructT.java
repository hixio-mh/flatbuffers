// automatically generated by the FlatBuffers compiler, do not modify

package MyGame.Example;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

public class NestedStructT {
  private int[] a;
  private byte b;
  private byte[] c;
  private long[] d;

  public int[] getA() { return a; }

  public void setA(int[] a) { if (a != null && a.length == 2) this.a = a; }

  public byte getB() { return b; }

  public void setB(byte b) { this.b = b; }

  public byte[] getC() { return c; }

  public void setC(byte[] c) { if (c != null && c.length == 2) this.c = c; }

  public long[] getD() { return d; }

  public void setD(long[] d) { if (d != null && d.length == 2) this.d = d; }


  public NestedStructT() {
    this.a = new int[2];
    this.b = 0;
    this.c = new byte[2];
    this.d = new long[2];
  }
}

