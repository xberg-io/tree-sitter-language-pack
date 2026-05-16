// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'lib.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$DocstringFormat {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DocstringFormat);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'DocstringFormat()';
}


}

/// @nodoc
class $DocstringFormatCopyWith<$Res>  {
$DocstringFormatCopyWith(DocstringFormat _, $Res Function(DocstringFormat) __);
}


/// Adds pattern-matching-related methods to [DocstringFormat].
extension DocstringFormatPatterns on DocstringFormat {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( DocstringFormat_PythonTripleQuote value)?  pythonTripleQuote,TResult Function( DocstringFormat_JSDoc value)?  jsDoc,TResult Function( DocstringFormat_Rustdoc value)?  rustdoc,TResult Function( DocstringFormat_GoDoc value)?  goDoc,TResult Function( DocstringFormat_JavaDoc value)?  javaDoc,TResult Function( DocstringFormat_Other value)?  other,required TResult orElse(),}){
final _that = this;
switch (_that) {
case DocstringFormat_PythonTripleQuote() when pythonTripleQuote != null:
return pythonTripleQuote(_that);case DocstringFormat_JSDoc() when jsDoc != null:
return jsDoc(_that);case DocstringFormat_Rustdoc() when rustdoc != null:
return rustdoc(_that);case DocstringFormat_GoDoc() when goDoc != null:
return goDoc(_that);case DocstringFormat_JavaDoc() when javaDoc != null:
return javaDoc(_that);case DocstringFormat_Other() when other != null:
return other(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( DocstringFormat_PythonTripleQuote value)  pythonTripleQuote,required TResult Function( DocstringFormat_JSDoc value)  jsDoc,required TResult Function( DocstringFormat_Rustdoc value)  rustdoc,required TResult Function( DocstringFormat_GoDoc value)  goDoc,required TResult Function( DocstringFormat_JavaDoc value)  javaDoc,required TResult Function( DocstringFormat_Other value)  other,}){
final _that = this;
switch (_that) {
case DocstringFormat_PythonTripleQuote():
return pythonTripleQuote(_that);case DocstringFormat_JSDoc():
return jsDoc(_that);case DocstringFormat_Rustdoc():
return rustdoc(_that);case DocstringFormat_GoDoc():
return goDoc(_that);case DocstringFormat_JavaDoc():
return javaDoc(_that);case DocstringFormat_Other():
return other(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( DocstringFormat_PythonTripleQuote value)?  pythonTripleQuote,TResult? Function( DocstringFormat_JSDoc value)?  jsDoc,TResult? Function( DocstringFormat_Rustdoc value)?  rustdoc,TResult? Function( DocstringFormat_GoDoc value)?  goDoc,TResult? Function( DocstringFormat_JavaDoc value)?  javaDoc,TResult? Function( DocstringFormat_Other value)?  other,}){
final _that = this;
switch (_that) {
case DocstringFormat_PythonTripleQuote() when pythonTripleQuote != null:
return pythonTripleQuote(_that);case DocstringFormat_JSDoc() when jsDoc != null:
return jsDoc(_that);case DocstringFormat_Rustdoc() when rustdoc != null:
return rustdoc(_that);case DocstringFormat_GoDoc() when goDoc != null:
return goDoc(_that);case DocstringFormat_JavaDoc() when javaDoc != null:
return javaDoc(_that);case DocstringFormat_Other() when other != null:
return other(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  pythonTripleQuote,TResult Function()?  jsDoc,TResult Function()?  rustdoc,TResult Function()?  goDoc,TResult Function()?  javaDoc,TResult Function( String field0)?  other,required TResult orElse(),}) {final _that = this;
switch (_that) {
case DocstringFormat_PythonTripleQuote() when pythonTripleQuote != null:
return pythonTripleQuote();case DocstringFormat_JSDoc() when jsDoc != null:
return jsDoc();case DocstringFormat_Rustdoc() when rustdoc != null:
return rustdoc();case DocstringFormat_GoDoc() when goDoc != null:
return goDoc();case DocstringFormat_JavaDoc() when javaDoc != null:
return javaDoc();case DocstringFormat_Other() when other != null:
return other(_that.field0);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  pythonTripleQuote,required TResult Function()  jsDoc,required TResult Function()  rustdoc,required TResult Function()  goDoc,required TResult Function()  javaDoc,required TResult Function( String field0)  other,}) {final _that = this;
switch (_that) {
case DocstringFormat_PythonTripleQuote():
return pythonTripleQuote();case DocstringFormat_JSDoc():
return jsDoc();case DocstringFormat_Rustdoc():
return rustdoc();case DocstringFormat_GoDoc():
return goDoc();case DocstringFormat_JavaDoc():
return javaDoc();case DocstringFormat_Other():
return other(_that.field0);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  pythonTripleQuote,TResult? Function()?  jsDoc,TResult? Function()?  rustdoc,TResult? Function()?  goDoc,TResult? Function()?  javaDoc,TResult? Function( String field0)?  other,}) {final _that = this;
switch (_that) {
case DocstringFormat_PythonTripleQuote() when pythonTripleQuote != null:
return pythonTripleQuote();case DocstringFormat_JSDoc() when jsDoc != null:
return jsDoc();case DocstringFormat_Rustdoc() when rustdoc != null:
return rustdoc();case DocstringFormat_GoDoc() when goDoc != null:
return goDoc();case DocstringFormat_JavaDoc() when javaDoc != null:
return javaDoc();case DocstringFormat_Other() when other != null:
return other(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class DocstringFormat_PythonTripleQuote extends DocstringFormat {
  const DocstringFormat_PythonTripleQuote(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DocstringFormat_PythonTripleQuote);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'DocstringFormat.pythonTripleQuote()';
}


}




/// @nodoc


class DocstringFormat_JSDoc extends DocstringFormat {
  const DocstringFormat_JSDoc(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DocstringFormat_JSDoc);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'DocstringFormat.jsDoc()';
}


}




/// @nodoc


class DocstringFormat_Rustdoc extends DocstringFormat {
  const DocstringFormat_Rustdoc(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DocstringFormat_Rustdoc);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'DocstringFormat.rustdoc()';
}


}




/// @nodoc


class DocstringFormat_GoDoc extends DocstringFormat {
  const DocstringFormat_GoDoc(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DocstringFormat_GoDoc);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'DocstringFormat.goDoc()';
}


}




/// @nodoc


class DocstringFormat_JavaDoc extends DocstringFormat {
  const DocstringFormat_JavaDoc(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DocstringFormat_JavaDoc);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'DocstringFormat.javaDoc()';
}


}




/// @nodoc


class DocstringFormat_Other extends DocstringFormat {
  const DocstringFormat_Other({required this.field0}): super._();


 final  String field0;

/// Create a copy of DocstringFormat
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$DocstringFormat_OtherCopyWith<DocstringFormat_Other> get copyWith => _$DocstringFormat_OtherCopyWithImpl<DocstringFormat_Other>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is DocstringFormat_Other&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'DocstringFormat.other(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $DocstringFormat_OtherCopyWith<$Res> implements $DocstringFormatCopyWith<$Res> {
  factory $DocstringFormat_OtherCopyWith(DocstringFormat_Other value, $Res Function(DocstringFormat_Other) _then) = _$DocstringFormat_OtherCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$DocstringFormat_OtherCopyWithImpl<$Res>
    implements $DocstringFormat_OtherCopyWith<$Res> {
  _$DocstringFormat_OtherCopyWithImpl(this._self, this._then);

  final DocstringFormat_Other _self;
  final $Res Function(DocstringFormat_Other) _then;

/// Create a copy of DocstringFormat
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(DocstringFormat_Other(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$StructureKind {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StructureKind);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StructureKind()';
}


}

/// @nodoc
class $StructureKindCopyWith<$Res>  {
$StructureKindCopyWith(StructureKind _, $Res Function(StructureKind) __);
}


/// Adds pattern-matching-related methods to [StructureKind].
extension StructureKindPatterns on StructureKind {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( StructureKind_Function value)?  function,TResult Function( StructureKind_Method value)?  method,TResult Function( StructureKind_Class value)?  class_,TResult Function( StructureKind_Struct value)?  struct,TResult Function( StructureKind_Interface value)?  interface_,TResult Function( StructureKind_Enum value)?  enum_,TResult Function( StructureKind_Module value)?  module,TResult Function( StructureKind_Trait value)?  trait,TResult Function( StructureKind_Impl value)?  impl,TResult Function( StructureKind_Namespace value)?  namespace,TResult Function( StructureKind_Other value)?  other,required TResult orElse(),}){
final _that = this;
switch (_that) {
case StructureKind_Function() when function != null:
return function(_that);case StructureKind_Method() when method != null:
return method(_that);case StructureKind_Class() when class_ != null:
return class_(_that);case StructureKind_Struct() when struct != null:
return struct(_that);case StructureKind_Interface() when interface_ != null:
return interface_(_that);case StructureKind_Enum() when enum_ != null:
return enum_(_that);case StructureKind_Module() when module != null:
return module(_that);case StructureKind_Trait() when trait != null:
return trait(_that);case StructureKind_Impl() when impl != null:
return impl(_that);case StructureKind_Namespace() when namespace != null:
return namespace(_that);case StructureKind_Other() when other != null:
return other(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( StructureKind_Function value)  function,required TResult Function( StructureKind_Method value)  method,required TResult Function( StructureKind_Class value)  class_,required TResult Function( StructureKind_Struct value)  struct,required TResult Function( StructureKind_Interface value)  interface_,required TResult Function( StructureKind_Enum value)  enum_,required TResult Function( StructureKind_Module value)  module,required TResult Function( StructureKind_Trait value)  trait,required TResult Function( StructureKind_Impl value)  impl,required TResult Function( StructureKind_Namespace value)  namespace,required TResult Function( StructureKind_Other value)  other,}){
final _that = this;
switch (_that) {
case StructureKind_Function():
return function(_that);case StructureKind_Method():
return method(_that);case StructureKind_Class():
return class_(_that);case StructureKind_Struct():
return struct(_that);case StructureKind_Interface():
return interface_(_that);case StructureKind_Enum():
return enum_(_that);case StructureKind_Module():
return module(_that);case StructureKind_Trait():
return trait(_that);case StructureKind_Impl():
return impl(_that);case StructureKind_Namespace():
return namespace(_that);case StructureKind_Other():
return other(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( StructureKind_Function value)?  function,TResult? Function( StructureKind_Method value)?  method,TResult? Function( StructureKind_Class value)?  class_,TResult? Function( StructureKind_Struct value)?  struct,TResult? Function( StructureKind_Interface value)?  interface_,TResult? Function( StructureKind_Enum value)?  enum_,TResult? Function( StructureKind_Module value)?  module,TResult? Function( StructureKind_Trait value)?  trait,TResult? Function( StructureKind_Impl value)?  impl,TResult? Function( StructureKind_Namespace value)?  namespace,TResult? Function( StructureKind_Other value)?  other,}){
final _that = this;
switch (_that) {
case StructureKind_Function() when function != null:
return function(_that);case StructureKind_Method() when method != null:
return method(_that);case StructureKind_Class() when class_ != null:
return class_(_that);case StructureKind_Struct() when struct != null:
return struct(_that);case StructureKind_Interface() when interface_ != null:
return interface_(_that);case StructureKind_Enum() when enum_ != null:
return enum_(_that);case StructureKind_Module() when module != null:
return module(_that);case StructureKind_Trait() when trait != null:
return trait(_that);case StructureKind_Impl() when impl != null:
return impl(_that);case StructureKind_Namespace() when namespace != null:
return namespace(_that);case StructureKind_Other() when other != null:
return other(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  function,TResult Function()?  method,TResult Function()?  class_,TResult Function()?  struct,TResult Function()?  interface_,TResult Function()?  enum_,TResult Function()?  module,TResult Function()?  trait,TResult Function()?  impl,TResult Function()?  namespace,TResult Function( String field0)?  other,required TResult orElse(),}) {final _that = this;
switch (_that) {
case StructureKind_Function() when function != null:
return function();case StructureKind_Method() when method != null:
return method();case StructureKind_Class() when class_ != null:
return class_();case StructureKind_Struct() when struct != null:
return struct();case StructureKind_Interface() when interface_ != null:
return interface_();case StructureKind_Enum() when enum_ != null:
return enum_();case StructureKind_Module() when module != null:
return module();case StructureKind_Trait() when trait != null:
return trait();case StructureKind_Impl() when impl != null:
return impl();case StructureKind_Namespace() when namespace != null:
return namespace();case StructureKind_Other() when other != null:
return other(_that.field0);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  function,required TResult Function()  method,required TResult Function()  class_,required TResult Function()  struct,required TResult Function()  interface_,required TResult Function()  enum_,required TResult Function()  module,required TResult Function()  trait,required TResult Function()  impl,required TResult Function()  namespace,required TResult Function( String field0)  other,}) {final _that = this;
switch (_that) {
case StructureKind_Function():
return function();case StructureKind_Method():
return method();case StructureKind_Class():
return class_();case StructureKind_Struct():
return struct();case StructureKind_Interface():
return interface_();case StructureKind_Enum():
return enum_();case StructureKind_Module():
return module();case StructureKind_Trait():
return trait();case StructureKind_Impl():
return impl();case StructureKind_Namespace():
return namespace();case StructureKind_Other():
return other(_that.field0);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  function,TResult? Function()?  method,TResult? Function()?  class_,TResult? Function()?  struct,TResult? Function()?  interface_,TResult? Function()?  enum_,TResult? Function()?  module,TResult? Function()?  trait,TResult? Function()?  impl,TResult? Function()?  namespace,TResult? Function( String field0)?  other,}) {final _that = this;
switch (_that) {
case StructureKind_Function() when function != null:
return function();case StructureKind_Method() when method != null:
return method();case StructureKind_Class() when class_ != null:
return class_();case StructureKind_Struct() when struct != null:
return struct();case StructureKind_Interface() when interface_ != null:
return interface_();case StructureKind_Enum() when enum_ != null:
return enum_();case StructureKind_Module() when module != null:
return module();case StructureKind_Trait() when trait != null:
return trait();case StructureKind_Impl() when impl != null:
return impl();case StructureKind_Namespace() when namespace != null:
return namespace();case StructureKind_Other() when other != null:
return other(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class StructureKind_Function extends StructureKind {
  const StructureKind_Function(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StructureKind_Function);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StructureKind.function()';
}


}




/// @nodoc


class StructureKind_Method extends StructureKind {
  const StructureKind_Method(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StructureKind_Method);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StructureKind.method()';
}


}




/// @nodoc


class StructureKind_Class extends StructureKind {
  const StructureKind_Class(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StructureKind_Class);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StructureKind.class_()';
}


}




/// @nodoc


class StructureKind_Struct extends StructureKind {
  const StructureKind_Struct(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StructureKind_Struct);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StructureKind.struct()';
}


}




/// @nodoc


class StructureKind_Interface extends StructureKind {
  const StructureKind_Interface(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StructureKind_Interface);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StructureKind.interface_()';
}


}




/// @nodoc


class StructureKind_Enum extends StructureKind {
  const StructureKind_Enum(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StructureKind_Enum);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StructureKind.enum_()';
}


}




/// @nodoc


class StructureKind_Module extends StructureKind {
  const StructureKind_Module(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StructureKind_Module);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StructureKind.module()';
}


}




/// @nodoc


class StructureKind_Trait extends StructureKind {
  const StructureKind_Trait(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StructureKind_Trait);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StructureKind.trait()';
}


}




/// @nodoc


class StructureKind_Impl extends StructureKind {
  const StructureKind_Impl(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StructureKind_Impl);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StructureKind.impl()';
}


}




/// @nodoc


class StructureKind_Namespace extends StructureKind {
  const StructureKind_Namespace(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StructureKind_Namespace);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'StructureKind.namespace()';
}


}




/// @nodoc


class StructureKind_Other extends StructureKind {
  const StructureKind_Other({required this.field0}): super._();


 final  String field0;

/// Create a copy of StructureKind
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$StructureKind_OtherCopyWith<StructureKind_Other> get copyWith => _$StructureKind_OtherCopyWithImpl<StructureKind_Other>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is StructureKind_Other&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'StructureKind.other(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $StructureKind_OtherCopyWith<$Res> implements $StructureKindCopyWith<$Res> {
  factory $StructureKind_OtherCopyWith(StructureKind_Other value, $Res Function(StructureKind_Other) _then) = _$StructureKind_OtherCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$StructureKind_OtherCopyWithImpl<$Res>
    implements $StructureKind_OtherCopyWith<$Res> {
  _$StructureKind_OtherCopyWithImpl(this._self, this._then);

  final StructureKind_Other _self;
  final $Res Function(StructureKind_Other) _then;

/// Create a copy of StructureKind
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(StructureKind_Other(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$SymbolKind {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SymbolKind);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'SymbolKind()';
}


}

/// @nodoc
class $SymbolKindCopyWith<$Res>  {
$SymbolKindCopyWith(SymbolKind _, $Res Function(SymbolKind) __);
}


/// Adds pattern-matching-related methods to [SymbolKind].
extension SymbolKindPatterns on SymbolKind {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( SymbolKind_Variable value)?  variable,TResult Function( SymbolKind_Constant value)?  constant,TResult Function( SymbolKind_Function value)?  function,TResult Function( SymbolKind_Class value)?  class_,TResult Function( SymbolKind_Type value)?  type,TResult Function( SymbolKind_Interface value)?  interface_,TResult Function( SymbolKind_Enum value)?  enum_,TResult Function( SymbolKind_Module value)?  module,TResult Function( SymbolKind_Other value)?  other,required TResult orElse(),}){
final _that = this;
switch (_that) {
case SymbolKind_Variable() when variable != null:
return variable(_that);case SymbolKind_Constant() when constant != null:
return constant(_that);case SymbolKind_Function() when function != null:
return function(_that);case SymbolKind_Class() when class_ != null:
return class_(_that);case SymbolKind_Type() when type != null:
return type(_that);case SymbolKind_Interface() when interface_ != null:
return interface_(_that);case SymbolKind_Enum() when enum_ != null:
return enum_(_that);case SymbolKind_Module() when module != null:
return module(_that);case SymbolKind_Other() when other != null:
return other(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( SymbolKind_Variable value)  variable,required TResult Function( SymbolKind_Constant value)  constant,required TResult Function( SymbolKind_Function value)  function,required TResult Function( SymbolKind_Class value)  class_,required TResult Function( SymbolKind_Type value)  type,required TResult Function( SymbolKind_Interface value)  interface_,required TResult Function( SymbolKind_Enum value)  enum_,required TResult Function( SymbolKind_Module value)  module,required TResult Function( SymbolKind_Other value)  other,}){
final _that = this;
switch (_that) {
case SymbolKind_Variable():
return variable(_that);case SymbolKind_Constant():
return constant(_that);case SymbolKind_Function():
return function(_that);case SymbolKind_Class():
return class_(_that);case SymbolKind_Type():
return type(_that);case SymbolKind_Interface():
return interface_(_that);case SymbolKind_Enum():
return enum_(_that);case SymbolKind_Module():
return module(_that);case SymbolKind_Other():
return other(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( SymbolKind_Variable value)?  variable,TResult? Function( SymbolKind_Constant value)?  constant,TResult? Function( SymbolKind_Function value)?  function,TResult? Function( SymbolKind_Class value)?  class_,TResult? Function( SymbolKind_Type value)?  type,TResult? Function( SymbolKind_Interface value)?  interface_,TResult? Function( SymbolKind_Enum value)?  enum_,TResult? Function( SymbolKind_Module value)?  module,TResult? Function( SymbolKind_Other value)?  other,}){
final _that = this;
switch (_that) {
case SymbolKind_Variable() when variable != null:
return variable(_that);case SymbolKind_Constant() when constant != null:
return constant(_that);case SymbolKind_Function() when function != null:
return function(_that);case SymbolKind_Class() when class_ != null:
return class_(_that);case SymbolKind_Type() when type != null:
return type(_that);case SymbolKind_Interface() when interface_ != null:
return interface_(_that);case SymbolKind_Enum() when enum_ != null:
return enum_(_that);case SymbolKind_Module() when module != null:
return module(_that);case SymbolKind_Other() when other != null:
return other(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  variable,TResult Function()?  constant,TResult Function()?  function,TResult Function()?  class_,TResult Function()?  type,TResult Function()?  interface_,TResult Function()?  enum_,TResult Function()?  module,TResult Function( String field0)?  other,required TResult orElse(),}) {final _that = this;
switch (_that) {
case SymbolKind_Variable() when variable != null:
return variable();case SymbolKind_Constant() when constant != null:
return constant();case SymbolKind_Function() when function != null:
return function();case SymbolKind_Class() when class_ != null:
return class_();case SymbolKind_Type() when type != null:
return type();case SymbolKind_Interface() when interface_ != null:
return interface_();case SymbolKind_Enum() when enum_ != null:
return enum_();case SymbolKind_Module() when module != null:
return module();case SymbolKind_Other() when other != null:
return other(_that.field0);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  variable,required TResult Function()  constant,required TResult Function()  function,required TResult Function()  class_,required TResult Function()  type,required TResult Function()  interface_,required TResult Function()  enum_,required TResult Function()  module,required TResult Function( String field0)  other,}) {final _that = this;
switch (_that) {
case SymbolKind_Variable():
return variable();case SymbolKind_Constant():
return constant();case SymbolKind_Function():
return function();case SymbolKind_Class():
return class_();case SymbolKind_Type():
return type();case SymbolKind_Interface():
return interface_();case SymbolKind_Enum():
return enum_();case SymbolKind_Module():
return module();case SymbolKind_Other():
return other(_that.field0);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  variable,TResult? Function()?  constant,TResult? Function()?  function,TResult? Function()?  class_,TResult? Function()?  type,TResult? Function()?  interface_,TResult? Function()?  enum_,TResult? Function()?  module,TResult? Function( String field0)?  other,}) {final _that = this;
switch (_that) {
case SymbolKind_Variable() when variable != null:
return variable();case SymbolKind_Constant() when constant != null:
return constant();case SymbolKind_Function() when function != null:
return function();case SymbolKind_Class() when class_ != null:
return class_();case SymbolKind_Type() when type != null:
return type();case SymbolKind_Interface() when interface_ != null:
return interface_();case SymbolKind_Enum() when enum_ != null:
return enum_();case SymbolKind_Module() when module != null:
return module();case SymbolKind_Other() when other != null:
return other(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class SymbolKind_Variable extends SymbolKind {
  const SymbolKind_Variable(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SymbolKind_Variable);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'SymbolKind.variable()';
}


}




/// @nodoc


class SymbolKind_Constant extends SymbolKind {
  const SymbolKind_Constant(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SymbolKind_Constant);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'SymbolKind.constant()';
}


}




/// @nodoc


class SymbolKind_Function extends SymbolKind {
  const SymbolKind_Function(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SymbolKind_Function);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'SymbolKind.function()';
}


}




/// @nodoc


class SymbolKind_Class extends SymbolKind {
  const SymbolKind_Class(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SymbolKind_Class);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'SymbolKind.class_()';
}


}




/// @nodoc


class SymbolKind_Type extends SymbolKind {
  const SymbolKind_Type(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SymbolKind_Type);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'SymbolKind.type()';
}


}




/// @nodoc


class SymbolKind_Interface extends SymbolKind {
  const SymbolKind_Interface(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SymbolKind_Interface);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'SymbolKind.interface_()';
}


}




/// @nodoc


class SymbolKind_Enum extends SymbolKind {
  const SymbolKind_Enum(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SymbolKind_Enum);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'SymbolKind.enum_()';
}


}




/// @nodoc


class SymbolKind_Module extends SymbolKind {
  const SymbolKind_Module(): super._();







@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SymbolKind_Module);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'SymbolKind.module()';
}


}




/// @nodoc


class SymbolKind_Other extends SymbolKind {
  const SymbolKind_Other({required this.field0}): super._();


 final  String field0;

/// Create a copy of SymbolKind
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$SymbolKind_OtherCopyWith<SymbolKind_Other> get copyWith => _$SymbolKind_OtherCopyWithImpl<SymbolKind_Other>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is SymbolKind_Other&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'SymbolKind.other(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $SymbolKind_OtherCopyWith<$Res> implements $SymbolKindCopyWith<$Res> {
  factory $SymbolKind_OtherCopyWith(SymbolKind_Other value, $Res Function(SymbolKind_Other) _then) = _$SymbolKind_OtherCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$SymbolKind_OtherCopyWithImpl<$Res>
    implements $SymbolKind_OtherCopyWith<$Res> {
  _$SymbolKind_OtherCopyWithImpl(this._self, this._then);

  final SymbolKind_Other _self;
  final $Res Function(SymbolKind_Other) _then;

/// Create a copy of SymbolKind
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(SymbolKind_Other(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
