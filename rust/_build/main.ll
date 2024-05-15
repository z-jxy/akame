; ModuleID = 'main'
source_filename = "main"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"

@format_str_s_ = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@format_str_d = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@format_str_s = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1
@argc_global = global i32 0
@argv_global = global ptr null

declare i8 @printf(ptr, ...)

define void @print(ptr %0) {
entry:
  %printf_call = call i8 (ptr, ...) @printf(ptr @format_str_s_, ptr %0)
  ret void
}

define i32 @printd(i32 %0) {
entry:
  %calltmp = call i8 (ptr, ...) @printf(ptr @format_str_d, i32 %0)
  ret i32 %0
}

define i32 @main(i32 %0, ptr %1) {
entry:
  store i32 %0, ptr @argc_global, align 4
  store ptr %1, ptr @argv_global, align 8
  %user_main_call = call i32 @_main()
  ret i32 0
}

define i32 @hello(i32 %0) {
entry:
  %addtmp = add i32 %0, 5
  ret i32 %addtmp
}

define i32 @_main() {
entry:
  %calltmp = call i32 @hello(i32 3)
  %calltmp1 = call i32 @printd(i32 %calltmp)
  ret i32 0
}
