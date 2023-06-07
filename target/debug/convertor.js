let temml = require('temml')
let process = require('process')

try {
console.log(temml.renderToString(process.argv.slice(2).join(' '), {throwOnError: true, displayMode: true}))
}
catch (e) {
    console.log(`Error: ${e}`)
    process.exit(1)
}