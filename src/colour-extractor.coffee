gm    = require('gm')
fs    = require('fs')
temp  = require('temp')

MAX_W      = 14
MIFF_START = 'comment={'
MIFF_END    = '\x0A}\x0A\x0C\x0A'

exports.topColours = (sourceFilename, sorted, cb) ->
  img = gm(sourceFilename)
  tmpFilename = temp.path({suffix: '.miff'})
  img.size((err, wh) ->
    console.log err if err
    
    ratio = wh.width/MAX_W
    w2 = wh.width/2
    h2 = wh.height/2
    img.noProfile()                               # Removes EXIF data
       .bitdepth(8)                               # Initial colour reduction, prob. smarter than our 'algorithm'
       .crop(w2, h2, w2/2, w2/2)                  # Center should be the most interesting
       .scale(Math.ceil(wh.height/ratio), MAX_W)  # Scales the image, histogram generation can take some time
       .write('histogram:' + tmpFilename, (err) ->
          console.log err if err

          histogram = ''
          miffRS = fs.createReadStream(tmpFilename, {encoding: 'utf8'})

          miffRS.addListener('data', (chunk) ->
            endDelimPos = chunk.indexOf(MIFF_END)

            if endDelimPos != -1
              histogram += chunk.slice(0, endDelimPos + MIFF_END.length)
              miffRS.destroy()
            else
              histogram += chunk
          )

          miffRS.addListener('close', ->
            fs.unlink(tmpFilename)

            histogram_start = histogram.indexOf(MIFF_START) + MIFF_START.length
            colours = reduceSimilar(clean(histogram.slice(histogram_start)
                                      .split('\n')
                                      .slice(1, -3)
                                      .map(parseHistogramLine)
                                      ))
            colours = colours.sort(sortByFrequency) if sorted
            cb(colours)
          )
        )
  )

exports.colourKey = (path, cb) ->
  exports.topColours(path, false, (xs) ->
    M = xs.length
    m = Math.ceil(M/2)

    cb([
      xs[0],   xs[1],   xs[2],
      xs[m-1], xs[m],   xs[m+1],
      xs[M-3], xs[M-2], xs[M-1]
    ])
  )

exports.rgb2hex = (r, g, b) ->
  rgb = if arguments.length is 1 then r else [r, g, b]
  '#' + rgb.map((x) -> (if x < 16 then '0' else '') + x.toString(16)).join('')

exports.hex2rgb = (xs) ->
  xs = xs.slice(1) if xs[0] is '#'
  [xs.slice(0, 2), xs.slice(2, -2), xs.slice(-2)].map((x) -> parseInt(x, 16))

# PRIVATE FUNCTIONS
include = (x, xs) ->
  xs.push(x) if xs.indexOf(x) is -1
  xs

clean = (xs) ->
  rs = []
  for x in xs
    rs.push(x) if x

  rs

sortByFrequency = ([a, _a2], [b, _b2]) ->
  return -1 if a > b
  return  1 if a < b
  return  0

distance = ([r1, g1, b1], [r2, g2, b2]) ->
  Math.sqrt(Math.pow(r1 - r2, 2) + Math.pow(g1 - g2, 2) + Math.pow(b1 - b2, 2))

###
Example line:
    f:  (rrr, ggg, bbb)   #rrggbb\n
    \   \                 \_____________ Hex code / "black" / "white"
     \   \______________________________ RGB triplet
      \_________________________________ Frequency at which colour appears
###
parseHistogramLine = (xs) ->
  xs = xs.trim().split(':')
  return null if xs.length != 2
  [+xs[0], xs[1].split('(')[1].split(')')[0].split(',').map((x) -> +x.trim())]

# Magic
reduceSimilar = (xs, r) ->
  minD = Infinity
  maxD = 0
  maxF = 0

  n = 0
  N = xs.length - 1
  tds = for x in xs
    break if n is N
    d = distance(x[1], xs[++n][1])
    minD = d if d < minD
    maxD = d if d > maxD
    d

  # geometric mean detects similar colours
  # appearing at lower frequencies
  avgD = Math.sqrt(minD * maxD)
  n = 0
  rs = []
  for d in tds
    if d > avgD
      include(xs[n], rs)
      maxF = xs[n][0] if xs[n][0] > maxF
    n++

  # Normalise values, [0, maxF] => [0, 1]
  rs.map(([f, c]) -> [f/maxF, c])
