spirv.module Logical GLSL450 requires #spirv.vce<v1.0, [Shader], []> {
  spirv.GlobalVariable @in {location = 0 : i32} : !spirv.ptr<vector<3xf32>, Input>
  spirv.GlobalVariable @out {location = 0 : i32} : !spirv.ptr<vector<4xf32>, Output>

  spirv.func @addFloat(%ptr1: !spirv.ptr<f32, Function>, %ptr2 : !spirv.ptr<f32, Function>) -> f32 "None" {
    %l1 = spirv.Load "Function" %ptr1 : f32
    %l2 = spirv.Load "Function" %ptr2 : f32
    %ret = spirv.FAdd %l1, %l2 : f32

    spirv.ReturnValue %ret : f32
  }

  spirv.func @appendFloat(%float: f32) -> !spirv.ptr<f32, Function>  "None" {
    %v1 = spirv.Variable : !spirv.ptr<f32, Function>
    spirv.Store "Function" %v1, %float : f32

    spirv.ReturnValue %v1 : !spirv.ptr<f32, Function>
  }


  spirv.func @vec3ToVec4(%vec3 : vector<3xf32>) -> vector<4xf32> "None" {
    %float_1 = spirv.Constant 1.0: f32
    %0 = spirv.CompositeExtract %vec3[0: i32]: vector<3xf32>
    %1 = spirv.CompositeExtract %vec3[1: i32]: vector<3xf32>
    %2 = spirv.CompositeExtract %vec3[2: i32]: vector<3xf32>

    %float_2 = spirv.Constant 3.0: f32
    spirv.FunctionCall @appendFloat(%float_2): (f32) -> !spirv.ptr<f32, Function>

    %constructed = spirv.CompositeConstruct %0, %1, %2, %float_1 : (f32,f32,f32,f32) -> vector<4xf32>

    spirv.ReturnValue %constructed : vector<4xf32>
  }

  spirv.func @main() -> () "None" {
    %in_ptr = spirv.mlir.addressof @in : !spirv.ptr<vector<3xf32>, Input>
    %out_ptr = spirv.mlir.addressof @out : !spirv.ptr<vector<4xf32>, Output>

    %0 = spirv.Variable : !spirv.ptr<!spirv.rtarray<f32>, Function>

    %in = spirv.Load "Input" %in_ptr : vector<3xf32>
    %out = spirv.FunctionCall @vec3ToVec4(%in) : (vector<3xf32>) ->  (vector<4xf32>)
    spirv.Store "Output" %out_ptr, %out : vector<4xf32>

    spirv.Return
  }
  spirv.EntryPoint "Fragment" @main
}
