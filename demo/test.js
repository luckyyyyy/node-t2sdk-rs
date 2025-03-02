const { Worker, isMainThread, parentPort } = require('worker_threads');
const t2sdk = require('../main.js');

const workerFunction = async () => {

  try {
    await t2sdk.init('lib/libt2sdk.so');

  } catch (e) {
    console.log(`Worker ${process.pid} init error:`, e);
  }

  const ins = new t2sdk.T2Sdk();

  ins.setConfig({
    t2sdk: {
      servers: '121.41.126.194:9399',
      license_file: 'license/license_hundsun.dat',
    },
  });

  try {
    const a = await ins.connect();
    console.log(`Worker ${process.pid} connected:`, a);
  } catch (e) {
    console.log(`Worker ${process.pid} connection error:`, e);
  }
  let i = 0;
  while (true) {
    try {

      i++;
      const cc = await ins.send({
        functionNo: 331100,
        subSystemNo: 1000,
        companyId: 91000,
      }, JSON.stringify({
        op_branch_no: 0,
        op_entrust_way: "7",
        op_station: "0",
        branch_no: 22,
        input_content: "1",
        account_content: "70960008",
        content_type: "",
        password: "111111",
        password_type: "",
      }));

      console.log(`Worker ${process.pid} error info:`, cc.getErrorInfo());
      console.log(`Worker ${process.pid} buffer:`, cc.getBuff());
    } catch (e) {
      console.log(`Worker ${process.pid} send error:`, e.message);
    }
    console.log(`Worker ${process.pid} send:`, i);
  }

  setInterval(() => {
    console.log(`Worker ${process.pid} is connected:`, ins.isConnected);
  }, 5000);
};
// workerFunction();

if (isMainThread) {
  // 主线程代码，创建两个 worker
  const worker1 = new Worker(__filename);
  // const worker2 = new Worker(__filename);

  worker1.on('message', (msg) => console.log('Message from worker 1:', msg));
  // worker2.on('message', (msg) => console.log('Message from worker 2:', msg));

  worker1.on('error', (error) => console.error('Worker 1 error:', error));
  // worker2.on('error', (error) => console.error('Worker 2 error:', error));

  worker1.on('exit', (code) => console.log('Worker 1 exited with code:', code));
  // worker2.on('exit', (code) => console.log('Worker 2 exited with code:', code));

} else {
  // 如果是 worker 线程，执行 workerFunction
  workerFunction();
}