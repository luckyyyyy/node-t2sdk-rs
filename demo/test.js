const t2sdk = require('../main.js');


(async () => {

  await t2sdk.init('lib/libt2sdk.so')

  console.log(t2sdk.getVersion())
  const packer = t2sdk.newPacker(0x20);
  const version = packer.getVersion();
  console.log(version)


  console.log(packer.beginPack())
  console.log(packer.addField("hello", "S".charCodeAt(), 10, 4))
  console.log(packer.addStr("test"))

  console.log(packer.getPackBuf(), 1)
  console.log(packer.endPack())
  const unpack = packer.unpack()
  const a = unpack.getPackBuf();
  console.log(a)

  console.log(t2sdk.getPackVersion(packer.getPackBuf()))

  const message = t2sdk.newBizMessage();
  message.setPacketType(t2sdk.REQUEST_PACKET);
  message.setFunction(123456);
  message.setContent(packer.getPackBuf());
  console.log(message.getFunction())
  console.log(message.getBuff())


  // const ins = new t2sdk.T2Sdk()
  // console.log(ins.config.setString('Section', 'Entry', 'Value'))
  // ins.config.save("hello.ini")
})();