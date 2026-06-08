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
mixin _$Error {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Error);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'Error()';
}


}

/// @nodoc
class $ErrorCopyWith<$Res>  {
$ErrorCopyWith(Error _, $Res Function(Error) __);
}


/// Adds pattern-matching-related methods to [Error].
extension ErrorPatterns on Error {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( Error_LanguageNotFound value)?  languageNotFound,TResult Function( Error_DynamicLoad value)?  dynamicLoad,TResult Function( Error_NullLanguagePointer value)?  nullLanguagePointer,TResult Function( Error_ParserSetup value)?  parserSetup,TResult Function( Error_LockPoisoned value)?  lockPoisoned,TResult Function( Error_Config value)?  config,TResult Function( Error_ParseFailed value)?  parseFailed,TResult Function( Error_QueryError value)?  queryError,TResult Function( Error_InvalidRange value)?  invalidRange,TResult Function( Error_Download value)?  download,TResult Function( Error_ChecksumMismatch value)?  checksumMismatch,TResult Function( Error_CacheLock value)?  cacheLock,required TResult orElse(),}){
final _that = this;
switch (_that) {
case Error_LanguageNotFound() when languageNotFound != null:
return languageNotFound(_that);case Error_DynamicLoad() when dynamicLoad != null:
return dynamicLoad(_that);case Error_NullLanguagePointer() when nullLanguagePointer != null:
return nullLanguagePointer(_that);case Error_ParserSetup() when parserSetup != null:
return parserSetup(_that);case Error_LockPoisoned() when lockPoisoned != null:
return lockPoisoned(_that);case Error_Config() when config != null:
return config(_that);case Error_ParseFailed() when parseFailed != null:
return parseFailed(_that);case Error_QueryError() when queryError != null:
return queryError(_that);case Error_InvalidRange() when invalidRange != null:
return invalidRange(_that);case Error_Download() when download != null:
return download(_that);case Error_ChecksumMismatch() when checksumMismatch != null:
return checksumMismatch(_that);case Error_CacheLock() when cacheLock != null:
return cacheLock(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( Error_LanguageNotFound value)  languageNotFound,required TResult Function( Error_DynamicLoad value)  dynamicLoad,required TResult Function( Error_NullLanguagePointer value)  nullLanguagePointer,required TResult Function( Error_ParserSetup value)  parserSetup,required TResult Function( Error_LockPoisoned value)  lockPoisoned,required TResult Function( Error_Config value)  config,required TResult Function( Error_ParseFailed value)  parseFailed,required TResult Function( Error_QueryError value)  queryError,required TResult Function( Error_InvalidRange value)  invalidRange,required TResult Function( Error_Download value)  download,required TResult Function( Error_ChecksumMismatch value)  checksumMismatch,required TResult Function( Error_CacheLock value)  cacheLock,}){
final _that = this;
switch (_that) {
case Error_LanguageNotFound():
return languageNotFound(_that);case Error_DynamicLoad():
return dynamicLoad(_that);case Error_NullLanguagePointer():
return nullLanguagePointer(_that);case Error_ParserSetup():
return parserSetup(_that);case Error_LockPoisoned():
return lockPoisoned(_that);case Error_Config():
return config(_that);case Error_ParseFailed():
return parseFailed(_that);case Error_QueryError():
return queryError(_that);case Error_InvalidRange():
return invalidRange(_that);case Error_Download():
return download(_that);case Error_ChecksumMismatch():
return checksumMismatch(_that);case Error_CacheLock():
return cacheLock(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( Error_LanguageNotFound value)?  languageNotFound,TResult? Function( Error_DynamicLoad value)?  dynamicLoad,TResult? Function( Error_NullLanguagePointer value)?  nullLanguagePointer,TResult? Function( Error_ParserSetup value)?  parserSetup,TResult? Function( Error_LockPoisoned value)?  lockPoisoned,TResult? Function( Error_Config value)?  config,TResult? Function( Error_ParseFailed value)?  parseFailed,TResult? Function( Error_QueryError value)?  queryError,TResult? Function( Error_InvalidRange value)?  invalidRange,TResult? Function( Error_Download value)?  download,TResult? Function( Error_ChecksumMismatch value)?  checksumMismatch,TResult? Function( Error_CacheLock value)?  cacheLock,}){
final _that = this;
switch (_that) {
case Error_LanguageNotFound() when languageNotFound != null:
return languageNotFound(_that);case Error_DynamicLoad() when dynamicLoad != null:
return dynamicLoad(_that);case Error_NullLanguagePointer() when nullLanguagePointer != null:
return nullLanguagePointer(_that);case Error_ParserSetup() when parserSetup != null:
return parserSetup(_that);case Error_LockPoisoned() when lockPoisoned != null:
return lockPoisoned(_that);case Error_Config() when config != null:
return config(_that);case Error_ParseFailed() when parseFailed != null:
return parseFailed(_that);case Error_QueryError() when queryError != null:
return queryError(_that);case Error_InvalidRange() when invalidRange != null:
return invalidRange(_that);case Error_Download() when download != null:
return download(_that);case Error_ChecksumMismatch() when checksumMismatch != null:
return checksumMismatch(_that);case Error_CacheLock() when cacheLock != null:
return cacheLock(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( String field0)?  languageNotFound,TResult Function( String field0)?  dynamicLoad,TResult Function( String field0)?  nullLanguagePointer,TResult Function( String field0)?  parserSetup,TResult Function( String field0)?  lockPoisoned,TResult Function( String field0)?  config,TResult Function()?  parseFailed,TResult Function( String field0)?  queryError,TResult Function( String field0)?  invalidRange,TResult Function( String field0)?  download,TResult Function( String file,  String expected,  String actual)?  checksumMismatch,TResult Function( String field0)?  cacheLock,required TResult orElse(),}) {final _that = this;
switch (_that) {
case Error_LanguageNotFound() when languageNotFound != null:
return languageNotFound(_that.field0);case Error_DynamicLoad() when dynamicLoad != null:
return dynamicLoad(_that.field0);case Error_NullLanguagePointer() when nullLanguagePointer != null:
return nullLanguagePointer(_that.field0);case Error_ParserSetup() when parserSetup != null:
return parserSetup(_that.field0);case Error_LockPoisoned() when lockPoisoned != null:
return lockPoisoned(_that.field0);case Error_Config() when config != null:
return config(_that.field0);case Error_ParseFailed() when parseFailed != null:
return parseFailed();case Error_QueryError() when queryError != null:
return queryError(_that.field0);case Error_InvalidRange() when invalidRange != null:
return invalidRange(_that.field0);case Error_Download() when download != null:
return download(_that.field0);case Error_ChecksumMismatch() when checksumMismatch != null:
return checksumMismatch(_that.file,_that.expected,_that.actual);case Error_CacheLock() when cacheLock != null:
return cacheLock(_that.field0);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( String field0)  languageNotFound,required TResult Function( String field0)  dynamicLoad,required TResult Function( String field0)  nullLanguagePointer,required TResult Function( String field0)  parserSetup,required TResult Function( String field0)  lockPoisoned,required TResult Function( String field0)  config,required TResult Function()  parseFailed,required TResult Function( String field0)  queryError,required TResult Function( String field0)  invalidRange,required TResult Function( String field0)  download,required TResult Function( String file,  String expected,  String actual)  checksumMismatch,required TResult Function( String field0)  cacheLock,}) {final _that = this;
switch (_that) {
case Error_LanguageNotFound():
return languageNotFound(_that.field0);case Error_DynamicLoad():
return dynamicLoad(_that.field0);case Error_NullLanguagePointer():
return nullLanguagePointer(_that.field0);case Error_ParserSetup():
return parserSetup(_that.field0);case Error_LockPoisoned():
return lockPoisoned(_that.field0);case Error_Config():
return config(_that.field0);case Error_ParseFailed():
return parseFailed();case Error_QueryError():
return queryError(_that.field0);case Error_InvalidRange():
return invalidRange(_that.field0);case Error_Download():
return download(_that.field0);case Error_ChecksumMismatch():
return checksumMismatch(_that.file,_that.expected,_that.actual);case Error_CacheLock():
return cacheLock(_that.field0);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( String field0)?  languageNotFound,TResult? Function( String field0)?  dynamicLoad,TResult? Function( String field0)?  nullLanguagePointer,TResult? Function( String field0)?  parserSetup,TResult? Function( String field0)?  lockPoisoned,TResult? Function( String field0)?  config,TResult? Function()?  parseFailed,TResult? Function( String field0)?  queryError,TResult? Function( String field0)?  invalidRange,TResult? Function( String field0)?  download,TResult? Function( String file,  String expected,  String actual)?  checksumMismatch,TResult? Function( String field0)?  cacheLock,}) {final _that = this;
switch (_that) {
case Error_LanguageNotFound() when languageNotFound != null:
return languageNotFound(_that.field0);case Error_DynamicLoad() when dynamicLoad != null:
return dynamicLoad(_that.field0);case Error_NullLanguagePointer() when nullLanguagePointer != null:
return nullLanguagePointer(_that.field0);case Error_ParserSetup() when parserSetup != null:
return parserSetup(_that.field0);case Error_LockPoisoned() when lockPoisoned != null:
return lockPoisoned(_that.field0);case Error_Config() when config != null:
return config(_that.field0);case Error_ParseFailed() when parseFailed != null:
return parseFailed();case Error_QueryError() when queryError != null:
return queryError(_that.field0);case Error_InvalidRange() when invalidRange != null:
return invalidRange(_that.field0);case Error_Download() when download != null:
return download(_that.field0);case Error_ChecksumMismatch() when checksumMismatch != null:
return checksumMismatch(_that.file,_that.expected,_that.actual);case Error_CacheLock() when cacheLock != null:
return cacheLock(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class Error_LanguageNotFound extends Error {
  const Error_LanguageNotFound({required this.field0}): super._();
  

 final  String field0;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Error_LanguageNotFoundCopyWith<Error_LanguageNotFound> get copyWith => _$Error_LanguageNotFoundCopyWithImpl<Error_LanguageNotFound>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Error_LanguageNotFound&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'Error.languageNotFound(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $Error_LanguageNotFoundCopyWith<$Res> implements $ErrorCopyWith<$Res> {
  factory $Error_LanguageNotFoundCopyWith(Error_LanguageNotFound value, $Res Function(Error_LanguageNotFound) _then) = _$Error_LanguageNotFoundCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$Error_LanguageNotFoundCopyWithImpl<$Res>
    implements $Error_LanguageNotFoundCopyWith<$Res> {
  _$Error_LanguageNotFoundCopyWithImpl(this._self, this._then);

  final Error_LanguageNotFound _self;
  final $Res Function(Error_LanguageNotFound) _then;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(Error_LanguageNotFound(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class Error_DynamicLoad extends Error {
  const Error_DynamicLoad({required this.field0}): super._();
  

 final  String field0;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Error_DynamicLoadCopyWith<Error_DynamicLoad> get copyWith => _$Error_DynamicLoadCopyWithImpl<Error_DynamicLoad>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Error_DynamicLoad&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'Error.dynamicLoad(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $Error_DynamicLoadCopyWith<$Res> implements $ErrorCopyWith<$Res> {
  factory $Error_DynamicLoadCopyWith(Error_DynamicLoad value, $Res Function(Error_DynamicLoad) _then) = _$Error_DynamicLoadCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$Error_DynamicLoadCopyWithImpl<$Res>
    implements $Error_DynamicLoadCopyWith<$Res> {
  _$Error_DynamicLoadCopyWithImpl(this._self, this._then);

  final Error_DynamicLoad _self;
  final $Res Function(Error_DynamicLoad) _then;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(Error_DynamicLoad(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class Error_NullLanguagePointer extends Error {
  const Error_NullLanguagePointer({required this.field0}): super._();
  

 final  String field0;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Error_NullLanguagePointerCopyWith<Error_NullLanguagePointer> get copyWith => _$Error_NullLanguagePointerCopyWithImpl<Error_NullLanguagePointer>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Error_NullLanguagePointer&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'Error.nullLanguagePointer(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $Error_NullLanguagePointerCopyWith<$Res> implements $ErrorCopyWith<$Res> {
  factory $Error_NullLanguagePointerCopyWith(Error_NullLanguagePointer value, $Res Function(Error_NullLanguagePointer) _then) = _$Error_NullLanguagePointerCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$Error_NullLanguagePointerCopyWithImpl<$Res>
    implements $Error_NullLanguagePointerCopyWith<$Res> {
  _$Error_NullLanguagePointerCopyWithImpl(this._self, this._then);

  final Error_NullLanguagePointer _self;
  final $Res Function(Error_NullLanguagePointer) _then;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(Error_NullLanguagePointer(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class Error_ParserSetup extends Error {
  const Error_ParserSetup({required this.field0}): super._();
  

 final  String field0;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Error_ParserSetupCopyWith<Error_ParserSetup> get copyWith => _$Error_ParserSetupCopyWithImpl<Error_ParserSetup>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Error_ParserSetup&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'Error.parserSetup(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $Error_ParserSetupCopyWith<$Res> implements $ErrorCopyWith<$Res> {
  factory $Error_ParserSetupCopyWith(Error_ParserSetup value, $Res Function(Error_ParserSetup) _then) = _$Error_ParserSetupCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$Error_ParserSetupCopyWithImpl<$Res>
    implements $Error_ParserSetupCopyWith<$Res> {
  _$Error_ParserSetupCopyWithImpl(this._self, this._then);

  final Error_ParserSetup _self;
  final $Res Function(Error_ParserSetup) _then;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(Error_ParserSetup(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class Error_LockPoisoned extends Error {
  const Error_LockPoisoned({required this.field0}): super._();
  

 final  String field0;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Error_LockPoisonedCopyWith<Error_LockPoisoned> get copyWith => _$Error_LockPoisonedCopyWithImpl<Error_LockPoisoned>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Error_LockPoisoned&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'Error.lockPoisoned(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $Error_LockPoisonedCopyWith<$Res> implements $ErrorCopyWith<$Res> {
  factory $Error_LockPoisonedCopyWith(Error_LockPoisoned value, $Res Function(Error_LockPoisoned) _then) = _$Error_LockPoisonedCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$Error_LockPoisonedCopyWithImpl<$Res>
    implements $Error_LockPoisonedCopyWith<$Res> {
  _$Error_LockPoisonedCopyWithImpl(this._self, this._then);

  final Error_LockPoisoned _self;
  final $Res Function(Error_LockPoisoned) _then;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(Error_LockPoisoned(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class Error_Config extends Error {
  const Error_Config({required this.field0}): super._();
  

 final  String field0;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Error_ConfigCopyWith<Error_Config> get copyWith => _$Error_ConfigCopyWithImpl<Error_Config>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Error_Config&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'Error.config(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $Error_ConfigCopyWith<$Res> implements $ErrorCopyWith<$Res> {
  factory $Error_ConfigCopyWith(Error_Config value, $Res Function(Error_Config) _then) = _$Error_ConfigCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$Error_ConfigCopyWithImpl<$Res>
    implements $Error_ConfigCopyWith<$Res> {
  _$Error_ConfigCopyWithImpl(this._self, this._then);

  final Error_Config _self;
  final $Res Function(Error_Config) _then;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(Error_Config(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class Error_ParseFailed extends Error {
  const Error_ParseFailed(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Error_ParseFailed);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'Error.parseFailed()';
}


}




/// @nodoc


class Error_QueryError extends Error {
  const Error_QueryError({required this.field0}): super._();
  

 final  String field0;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Error_QueryErrorCopyWith<Error_QueryError> get copyWith => _$Error_QueryErrorCopyWithImpl<Error_QueryError>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Error_QueryError&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'Error.queryError(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $Error_QueryErrorCopyWith<$Res> implements $ErrorCopyWith<$Res> {
  factory $Error_QueryErrorCopyWith(Error_QueryError value, $Res Function(Error_QueryError) _then) = _$Error_QueryErrorCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$Error_QueryErrorCopyWithImpl<$Res>
    implements $Error_QueryErrorCopyWith<$Res> {
  _$Error_QueryErrorCopyWithImpl(this._self, this._then);

  final Error_QueryError _self;
  final $Res Function(Error_QueryError) _then;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(Error_QueryError(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class Error_InvalidRange extends Error {
  const Error_InvalidRange({required this.field0}): super._();
  

 final  String field0;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Error_InvalidRangeCopyWith<Error_InvalidRange> get copyWith => _$Error_InvalidRangeCopyWithImpl<Error_InvalidRange>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Error_InvalidRange&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'Error.invalidRange(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $Error_InvalidRangeCopyWith<$Res> implements $ErrorCopyWith<$Res> {
  factory $Error_InvalidRangeCopyWith(Error_InvalidRange value, $Res Function(Error_InvalidRange) _then) = _$Error_InvalidRangeCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$Error_InvalidRangeCopyWithImpl<$Res>
    implements $Error_InvalidRangeCopyWith<$Res> {
  _$Error_InvalidRangeCopyWithImpl(this._self, this._then);

  final Error_InvalidRange _self;
  final $Res Function(Error_InvalidRange) _then;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(Error_InvalidRange(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class Error_Download extends Error {
  const Error_Download({required this.field0}): super._();
  

 final  String field0;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Error_DownloadCopyWith<Error_Download> get copyWith => _$Error_DownloadCopyWithImpl<Error_Download>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Error_Download&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'Error.download(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $Error_DownloadCopyWith<$Res> implements $ErrorCopyWith<$Res> {
  factory $Error_DownloadCopyWith(Error_Download value, $Res Function(Error_Download) _then) = _$Error_DownloadCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$Error_DownloadCopyWithImpl<$Res>
    implements $Error_DownloadCopyWith<$Res> {
  _$Error_DownloadCopyWithImpl(this._self, this._then);

  final Error_Download _self;
  final $Res Function(Error_Download) _then;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(Error_Download(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class Error_ChecksumMismatch extends Error {
  const Error_ChecksumMismatch({required this.file, required this.expected, required this.actual}): super._();
  

 final  String file;
 final  String expected;
 final  String actual;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Error_ChecksumMismatchCopyWith<Error_ChecksumMismatch> get copyWith => _$Error_ChecksumMismatchCopyWithImpl<Error_ChecksumMismatch>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Error_ChecksumMismatch&&(identical(other.file, file) || other.file == file)&&(identical(other.expected, expected) || other.expected == expected)&&(identical(other.actual, actual) || other.actual == actual));
}


@override
int get hashCode => Object.hash(runtimeType,file,expected,actual);

@override
String toString() {
  return 'Error.checksumMismatch(file: $file, expected: $expected, actual: $actual)';
}


}

/// @nodoc
abstract mixin class $Error_ChecksumMismatchCopyWith<$Res> implements $ErrorCopyWith<$Res> {
  factory $Error_ChecksumMismatchCopyWith(Error_ChecksumMismatch value, $Res Function(Error_ChecksumMismatch) _then) = _$Error_ChecksumMismatchCopyWithImpl;
@useResult
$Res call({
 String file, String expected, String actual
});




}
/// @nodoc
class _$Error_ChecksumMismatchCopyWithImpl<$Res>
    implements $Error_ChecksumMismatchCopyWith<$Res> {
  _$Error_ChecksumMismatchCopyWithImpl(this._self, this._then);

  final Error_ChecksumMismatch _self;
  final $Res Function(Error_ChecksumMismatch) _then;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? file = null,Object? expected = null,Object? actual = null,}) {
  return _then(Error_ChecksumMismatch(
file: null == file ? _self.file : file // ignore: cast_nullable_to_non_nullable
as String,expected: null == expected ? _self.expected : expected // ignore: cast_nullable_to_non_nullable
as String,actual: null == actual ? _self.actual : actual // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class Error_CacheLock extends Error {
  const Error_CacheLock({required this.field0}): super._();
  

 final  String field0;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$Error_CacheLockCopyWith<Error_CacheLock> get copyWith => _$Error_CacheLockCopyWithImpl<Error_CacheLock>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is Error_CacheLock&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'Error.cacheLock(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $Error_CacheLockCopyWith<$Res> implements $ErrorCopyWith<$Res> {
  factory $Error_CacheLockCopyWith(Error_CacheLock value, $Res Function(Error_CacheLock) _then) = _$Error_CacheLockCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$Error_CacheLockCopyWithImpl<$Res>
    implements $Error_CacheLockCopyWith<$Res> {
  _$Error_CacheLockCopyWithImpl(this._self, this._then);

  final Error_CacheLock _self;
  final $Res Function(Error_CacheLock) _then;

/// Create a copy of Error
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(Error_CacheLock(
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
