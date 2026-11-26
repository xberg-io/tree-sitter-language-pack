@extends('layouts.app')

@section('title', 'Home')

@section('content')
    <p>{{ $message }}</p>
    @include('partials.footer')
@endsection

@push('scripts')
    <script src="/app.js"></script>
@endpush
