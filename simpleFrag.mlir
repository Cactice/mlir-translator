spirv.module Logical GLSL450 requires #spirv.vce<v1.0, [Shader], []> {
  spirv.func @vec3ToVec4(%arg2 : vector<3xf32>) -> vector<4xf32> "None" {
    %int_1 = spirv.Constant 1.0: f32
    %1 = spirv.CompositeExtract %arg2[0 : i32] :vector<3xf32>
    %2 = spirv.CompositeExtract %arg2[1 : i32] :vector<3xf32>
    %3 = spirv.CompositeExtract %arg2[2: i32] :vector<3xf32>
    %4 = spirv.CompositeConstruct %1, %2, %3, %int_1 : (f32,f32,f32,f32) -> vector<4xf32>
    spirv.ReturnValue %4 : vector<4xf32>
  }

  spirv.func @frag_main(%arg0 : vector<3xf32>) -> vector<4xf32> "None" {
    %0 = spirv.FunctionCall @vec3ToVec4(%arg0) : (vector<3xf32>) ->  (vector<4xf32>)
    spirv.ReturnValue %0 : vector<4xf32>
  }
  spirv.EntryPoint "Fragment" @frag_main
}
