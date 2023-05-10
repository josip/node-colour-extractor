import test from 'ava'

import { topColorsHex, topColours } from '../index.js';

test('throws on missing file', async (t) => {
  await t.throwsAsync(async () => await topColours('./non-existing-file.png'));
});

test('identifies colors', async (t) => {
  const colors = await topColours('./__test__/sample.png');
  t.true(colors.length > 3, 'there are not enough colours');
  const uniqueColors = [...new Set(colors)];
  t.true(uniqueColors.length > 3, 'not enough unique colors');
});

test('identifies rgb colours', async (t) => {
  const [colour] = await topColours('./__test__/sample.png');
  t.is(colour.length, 3);
  t.true(colour.every(c => Number.isInteger(c) && c >= 0 && c <= 255));
});

test('identifies hex colours', async (t) => {
  const [colour] = await topColorsHex('./__test__/sample.png');
  t.is(typeof colour, 'string');
  t.true(colour.startsWith('#'));
  t.is(colour.length, 7);
  t.true(Number.isInteger(parseInt(colour.slice(1), 16)));
});
