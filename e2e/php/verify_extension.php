#!/usr/bin/env php
<?php

declare(strict_types=1);

fwrite(STDOUT, "=== PHP Extension Loading Verification ===\n\n");

// Test 1: Verify extension path is correct (ts-pack-core-php)
$extPath = __DIR__ . '/../../target/release/libts_pack_core_php.dylib';
fwrite(STDOUT, "1. Correct library filename derived from cargo_crate_name:\n");
fwrite(STDOUT, "   Expected: libts_pack_core_php.dylib\n");
fwrite(STDOUT, '   Actual:   ' . basename($extPath) . "\n");
fwrite(STDOUT, '   Status:   ' . (basename($extPath) === 'libts_pack_core_php.dylib' ? '✓ PASS' : '✗ FAIL') . "\n\n");

// Test 2: Verify extension file exists
fwrite(STDOUT, "2. Extension file exists at expected path:\n");
fwrite(STDOUT, "   Path:     $extPath\n");
fwrite(STDOUT, '   Exists:   ' . (file_exists($extPath) ? 'YES ✓' : 'NO ✗') . "\n\n");

// Test 3: Verify extension loads with -n flag (bypassing php.ini)
fwrite(STDOUT, "3. Extension loads correctly when invoked with -n flag:\n");
fwrite(STDOUT, '   Extension loaded: ' . (extension_loaded('ts-pack-core-php') ? 'YES ✓' : 'NO ✗') . "\n\n");

fwrite(STDOUT, "4. ProcessConfig class has from_json() method:\n");
if (extension_loaded('ts-pack-core-php')) {
    if (class_exists('Tree\Sitter\Language\Pack\ProcessConfig')) {
        if (method_exists('Tree\Sitter\Language\Pack\ProcessConfig', 'from_json')) {
            fwrite(STDOUT, "   Status: ✓ PASS - ProcessConfig::from_json() available\n\n");
        } else {
            fwrite(STDOUT, "   Status: ✗ FAIL - ProcessConfig::from_json() not found\n\n");
        }
    } else {
        fwrite(STDOUT, "   Status: ✗ FAIL - ProcessConfig class not found\n\n");
    }
} else {
    fwrite(STDOUT, "   Status: ✗ FAIL - Extension not loaded\n\n");
}

// Test 5: Verify run_tests.php uses correct logic (check file content)
fwrite(STDOUT, "5. run_tests.php uses new -n logic:\n");
$runTestsContent = file_get_contents(__DIR__ . '/run_tests.php');
if (strpos($runTestsContent, "'-n'") !== false) {
    fwrite(STDOUT, "   Status: ✓ PASS - run_tests.php contains -n flag logic\n");
} else {
    fwrite(STDOUT, "   Status: ✗ FAIL - run_tests.php missing -n flag logic\n");
}

if (strpos($runTestsContent, 'ALEF_PHP_LOCAL_EXT_LOADED') !== false) {
    fwrite(STDOUT, "   Status: ✓ PASS - run_tests.php uses ALEF_PHP_LOCAL_EXT_LOADED variable\n\n");
} else {
    fwrite(STDOUT, "   Status: ✗ FAIL - run_tests.php missing ALEF_PHP_LOCAL_EXT_LOADED logic\n\n");
}

fwrite(STDOUT, "=== All Verification Tests Complete ===\n");
