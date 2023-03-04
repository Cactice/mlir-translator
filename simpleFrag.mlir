spirv.module Logical GLSL450 requires #spirv.vce<v1.0, [Shader], []> {
  spirv.GlobalVariable @in {location = 0 : i32} : !spirv.ptr<vector<3xf32>, Input>
  spirv.GlobalVariable @out {location = 0 : i32} : !spirv.ptr<vector<4xf32>, Output>

  spirv.func @vec3ToVec4(%vec3 : vector<3xf32>) -> vector<4xf32> "None" {
    %int_1 = spirv.Constant 1.0: f32
    %1 = spirv.CompositeExtract %vec3[0 : i32] :vector<3xf32>
    %2 = spirv.CompositeExtract %vec3[1 : i32] :vector<3xf32>
    %3 = spirv.CompositeExtract %vec3[2: i32] :vector<3xf32>
    %4 = spirv.CompositeConstruct %1, %2, %3, %int_1 : (f32,f32,f32,f32) -> vector<4xf32>
    spirv.ReturnValue %4 : vector<4xf32>
  }

  spirv.func @main() -> () "None" {
    %in_ptr = spirv.mlir.addressof @in : !spirv.ptr<vector<3xf32>, Input>
    %out_ptr = spirv.mlir.addressof @out : !spirv.ptr<vector<4xf32>, Output>

    %in = spirv.Load "Input" %in_ptr : vector<3xf32>
    %vec4 = spirv.FunctionCall @vec3ToVec4(%in) : (vector<3xf32>) ->  (vector<4xf32>)
    spirv.Store "Output" %out_ptr, %vec4 : vector<4xf32>

    spirv.Return
  }
  spirv.EntryPoint "Fragment" @main
}
