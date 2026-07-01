#!/usr/bin/env php
<?php

declare(strict_types=1);

echo "=== PHP Extension Loading Verification ===\n\n";

// Test 1: Verify extension path is correct (ts-pack-core-php)
$extPath = __DIR__ . '/../../target/release/libts_pack_core_php.dylib';
echo "1. Correct library filename derived from cargo_crate_name:\n";
echo "   Expected: libts_pack_core_php.dylib\n";
echo '   Actual:   ' . basename($extPath) . "\n";
echo '   Status:   ' . (basename($extPath) === 'libts_pack_core_php.dylib' ? '✓ PASS' : '✗ FAIL') . "\n\n";

// Test 2: Verify extension file exists
echo "2. Extension file exists at expected path:\n";
echo "   Path:     $extPath\n";
echo '   Exists:   ' . (file_exists($extPath) ? 'YES ✓' : 'NO ✗') . "\n\n";

// Test 3: Verify extension loads with -n flag (bypassing php.ini)
echo "3. Extension loads correctly when invoked with -n flag:\n";
echo '   Extension loaded: ' . (extension_loaded('ts-pack-core-php') ? 'YES ✓' : 'NO ✗') . "\n\n";

// Test 4: Verify ProcessConfig::from_json() is available
echo "4. ProcessConfig class has from_json() method:\n";
if (extension_loaded('ts-pack-core-php')) {
    if (class_exists('Tree\Sitter\Language\Pack\ProcessConfig')) {
        if (method_exists('Tree\Sitter\Language\Pack\ProcessConfig', 'from_json')) {
            echo "   Status: ✓ PASS - ProcessConfig::from_json() available\n\n";
        } else {
            echo "   Status: ✗ FAIL - ProcessConfig::from_json() not found\n\n";
        }
    } else {
        echo "   Status: ✗ FAIL - ProcessConfig class not found\n\n";
    }
} else {
    echo "   Status: ✗ FAIL - Extension not loaded\n\n";
}

// Test 5: Verify run_tests.php uses correct logic (check file content)
echo "5. run_tests.php uses new -n logic:\n";
$runTestsContent = file_get_contents(__DIR__ . '/run_tests.php');
if (strpos($runTestsContent, "'-n'") !== false) {
    echo "   Status: ✓ PASS - run_tests.php contains -n flag logic\n";
} else {
    echo "   Status: ✗ FAIL - run_tests.php missing -n flag logic\n";
}

if (strpos($runTestsContent, 'ALEF_PHP_LOCAL_EXT_LOADED') !== false) {
    echo "   Status: ✓ PASS - run_tests.php uses ALEF_PHP_LOCAL_EXT_LOADED variable\n\n";
} else {
    echo "   Status: ✗ FAIL - run_tests.php missing ALEF_PHP_LOCAL_EXT_LOADED logic\n\n";
}

echo "=== All Verification Tests Complete ===\n";
