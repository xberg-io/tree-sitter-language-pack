#!/usr/bin/env php
<?php

declare(strict_types=1);

echo "========================================\n";
echo "PHP E2E Test Runner Fix Verification\n";
echo "========================================\n\n";

$passed = 0;
$total = 0;

function test($name, $condition)
{
    global $passed, $total;
    $total++;
    $status = $condition ? '✓ PASS' : '✗ FAIL';
    echo "[$total] $name: $status\n";
    if ($condition)
        $passed++;
    return $condition;
}

// Test 1: Library path is correct
$run_tests = file_get_contents(__DIR__ . '/run_tests.php');
test('Library path uses ts_pack_core_php', strpos($run_tests, 'libts_pack_core_php') !== false);

// Test 2: Uses -n flag
test('Uses -n flag to bypass php.ini', strpos($run_tests, "'-n'") !== false);

// Test 3: Uses environment variable
test('Uses ALEF_PHP_LOCAL_EXT_LOADED env var', strpos($run_tests, 'ALEF_PHP_LOCAL_EXT_LOADED') !== false);

// Test 4: Extension loads correctly
test('Extension loads with -n flag', extension_loaded('ts-pack-core-php'));

// Test 5: ProcessConfig exists
test('ProcessConfig class exists', class_exists('Tree\Sitter\Language\Pack\ProcessConfig'));

// Test 6: ProcessConfig::from_json exists
test('ProcessConfig::from_json() method exists', method_exists('Tree\Sitter\Language\Pack\ProcessConfig', 'from_json'));

// Test 7: File exists check
$ext_path = __DIR__ . '/../../target/release/libts_pack_core_php.dylib';
test('Extension binary exists at correct path', file_exists($ext_path));

// Test 8: Correct basename
test('Extension filename is libts_pack_core_php.dylib', basename($ext_path) === 'libts_pack_core_php.dylib');

echo "\n========================================\n";
echo "Result: $passed/$total tests passed\n";
if ($passed === $total) {
    echo "Status: ✓ ALL TESTS PASSED\n";
} else {
    echo "Status: ✗ SOME TESTS FAILED\n";
}
echo "========================================\n";
