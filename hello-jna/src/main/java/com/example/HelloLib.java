package com.example;

import com.sun.jna.Library;
import com.sun.jna.Structure;

import java.util.List;

public interface HelloLib extends Library {
    @Structure.FieldOrder("v")
    class SimpleStruct extends Structure {
        public static class ByValue extends SimpleStruct implements Structure.ByValue { }
        public static class ByReference extends SimpleStruct implements Structure.ByReference {}

        public int v;
    }

    // construct_ss_ref(v: u32) -> *mut SimpleStruct
    SimpleStruct construct_ss_ref(int v);

    // construct_ss_val(v: u32) -> SimpleStruct
    SimpleStruct.ByValue construct_ss_val(int v);

    // getv_by_ref(ss: *const SimpleStruct) -> u32
    int getv_by_ref(SimpleStruct ss);

    // getv_by_val(ss: SimpleStruct) -> u32
    int getv_by_val(SimpleStruct.ByValue ssv);

    // sum_by_refs(ss: *const *const SimpleStruct, len: usize) -> u32
    int sum_by_refs(SimpleStruct.ByReference[] ssrs, int length);

    // sum_by_vals(ss: *const SimpleStruct, len: usize) -> u32
    int sum_by_vals(SimpleStruct[] sss, int length);
}
