// RUN: mlir-translate -no-implicit-module -test-spirv-roundtrip %s | FileCheck %s

spirv.module Logical GLSL450 requires #spirv.vce<v1.0, [Shader], []> {
  spirv.func @math(%arg0 : vector<4xi32>, %arg1: vector<4xi32>) -> vector<4xi32> "None" {
    %0 = spirv.IAdd %arg0, %arg1 : vector<4xi32>
    spirv.ReturnValue %0 : vector<4xi32>
  }
  spirv.EntryPoint "Vertex" @math
}
