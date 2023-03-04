spirv.module Logical GLSL450 requires #spirv.vce<v1.0, [Shader], []> {
  spirv.func @vec3ToVec4(%arg2 : vector<3xi32>) -> vector<4xi32> "None" {
    %int_1 = spirv.Constant 1: i32
    %1 = spirv.CompositeExtract %arg2[0 : i32] :vector<3xi32>
    %2 = spirv.CompositeExtract %arg2[1 : i32] :vector<3xi32>
    %3 = spirv.CompositeExtract %arg2[2: i32] :vector<3xi32>
    %4 = spirv.CompositeConstruct %1, %2, %3, %int_1 : (i32,i32,i32,i32) -> vector<4xi32>
    spirv.ReturnValue %4 : vector<4xi32>
  }

  spirv.func @vert_main(%arg0 : vector<3xi32>) -> vector<4xi32> "None" {
    %0 = spirv.FunctionCall @vec3ToVec4(%arg0) : (vector<3xi32>) ->  (vector<4xi32>)
    spirv.ReturnValue %0 : vector<4xi32>
  }
  spirv.EntryPoint "Fragment" @vert_main
}
