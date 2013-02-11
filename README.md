# colour-extractor

Extract colour palettes from photos using Node.js.

## Installation

Is as simple as with any other Node.js module:

    $ npm install colour-extractor

NOTE: `colour-extractor` depends on [gm](http://aheckmann.github.com/gm/) module, which in turn depends on [GraphicsMagick](http://www.graphicsmagick.org).

## Usage

`colour-extractor` exports two functions:

    ce = require('colour-extractor')
    ce.topColours('Photos/Cats/01.jpg', true, function (colours) {
      console.log(colours);
    });

`topColours` function takes three arguments:

  * path to your photo,
  * `true` if you'd like the resulting array to be sorted by frequency,
    `false` if you'd like to get colours sorted as they appear in the photo (top-to-bottom),
  * a callback function.

Callback function will be passed an `Array` with RGB triplet of each colour and its frequency:

    [
      [1,   [46, 70, 118]],
      [0.3, [0,   0,   2]],
      [0.2, [12,  44,  11]]
    ]

The second function, `colourKey`, returns an array with nine colours, where each one can be mapped to a 3x3 box, ie. super-pixelised representation of the photo.

    ce.colourKey('Photos/Cats/999999.jpg', function (colours) {
      database.store('colour-keys', photoId, colours);
      res.send(colours);
      // render colours to user while they wait for the photo to load.
      // (or something equally brilliant)
    });


### Utilities

`colour-extractor` exports two more utility functions:

    > ce.rgb2hex(100, 10, 12);
    '#640a0c'
    > ce.rgb2hex([44, 44, 44]);
    '#2c2c2c'
    > ce.hex2rgb('#ffffff');
    [255, 255, 255]
    > ce.hex2rgb('45c092')
    [69, 192, 146]

## How does it work?

That's what I'd like to know as well! Anyhow, `colour-extractor` parses GraphicMagick's histogram, tries to detect similar colours and remove ones which appear less frequently than others.

If you happen to know an actual algorithm that deals with this sort of stuff, don't hesitate to contact me!

## Licence

Copyright (C) 2011 by Josip Lisec <Josip Of JLX.cc>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

  * The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

  * The Software shall not be used in preparation, baking and consumption of chocolate cupcakes.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

