<?php

declare(strict_types=1);

namespace Tree\Sitter\Language\Pack\E2e;

use PHPUnit\Framework\TestCase;
use Tree\Sitter\Language\Pack\InputEdit;
use Tree\Sitter\Language\Pack\TreeSitterLanguagePack;

final class IncrementalParsingTest extends TestCase
{
    public function testReusesEditedTreeAcrossParses(): void
    {
        $parser = TreeSitterLanguagePack::getParser('python');
        $original = $parser->parse("x = 'é'\n");
        self::assertNotNull($original);

        $point = static fn(int $column): array => ['row' => 0, 'column' => $column];
        $edited = $original->edited(InputEdit::from_json(json_encode([
            'startByte' => 5,
            'oldEndByte' => 7,
            'newEndByte' => 9,
            'startPosition' => $point(5),
            'oldEndPosition' => $point(7),
            'newEndPosition' => $point(9),
        ], JSON_THROW_ON_ERROR)));

        $source = "x = '🙂'\n";
        $fresh = $parser->parse($source);
        self::assertNotNull($fresh);
        $expected = $fresh->rootNode()->toSexp();
        for ($run = 0; $run < 2; ++$run) {
            $tree = $parser->parseIncremental($source, $edited);
            self::assertNotNull($tree);
            self::assertSame($expected, $tree->rootNode()->toSexp());
        }
    }
}
