const t2sdk = require('../main.js');


(async () => {

  await t2sdk.init('lib/libt2sdk.so')

  console.log(t2sdk.getVersion())
  const ins = new t2sdk.T2Sdk()
  console.log(ins.config.setString('Section', 'Entry', 'Value'))
  // ins.config.save("hello.ini")
})();