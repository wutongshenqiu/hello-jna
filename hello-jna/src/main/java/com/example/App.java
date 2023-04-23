package com.example;

import com.sun.jna.Native;
import com.sun.jna.Structure;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * Hello world!
 *
 */
public class App 
{
    public static void main( String[] args )
    {
        HelloLib helloLib = Native.load("../rs-libs/target/debug/libhello.so", HelloLib.class);

        {
            System.out.println("construct simple struct and return reference: " + helloLib.construct_ss_ref(1));
            System.out.println("construct simple struct and return value: " + helloLib.construct_ss_val(1));
        }

        {
            HelloLib.SimpleStruct ss = new HelloLib.SimpleStruct();
            HelloLib.SimpleStruct.ByValue ssv = new HelloLib.SimpleStruct.ByValue();
            ssv.v = ss.v = 1234;

            System.out.println("get value of simple struct by reference: " + helloLib.getv_by_ref(ss));
            System.out.println("get value of simple struct by value: " + helloLib.getv_by_val(ssv));
        }

        int length = 10;
        {
            List<HelloLib.SimpleStruct.ByReference> ssrs = new ArrayList<>();
            for (int i = 1; i <= length; i++) {
                HelloLib.SimpleStruct.ByReference ssr = new HelloLib.SimpleStruct.ByReference();
                ssr.v = i;
                ssrs.add(ssr);
            }
            System.out.println("get sum of simple structs by reference: " + helloLib.sum_by_refs(ssrs.toArray(new HelloLib.SimpleStruct.ByReference[0]), length));
        }

        {
            HelloLib.SimpleStruct[] sss = (HelloLib.SimpleStruct[]) new HelloLib.SimpleStruct().toArray(length);
            for (int i = 0; i < length; i++) {
                sss[i].v = i + 1;
            }
            System.out.println("get sum of simple structs by value: " + helloLib.sum_by_vals(sss, length));
        }

        {
            System.out.println("construct nested struct and return reference: " + helloLib.construct_ns_ref(1).s);
        }

        {
            System.out.println("construct nested struct reference and return reference: " + helloLib.construct_nsr_ref(1).s);
        }

        {
            int []vs = {1, 2, 3};
            HelloLib.StructArray sa = helloLib.construct_sa_ref(vs, vs.length);
            System.out.print("construct struct array and return reference: " + Arrays.toString(sa.ss.toArray(vs.length)));
        }
    }
}
