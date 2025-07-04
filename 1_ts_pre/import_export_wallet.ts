import bs58 from 'bs58'
import inquirer from 'inquirer';

async function base58_to_wallet() {
 const { base58Key } = await inquirer.prompt([
    {
      type: 'input',
      name: 'base58Key',
      message: 'Enter Phantom private key (base58):'
    }
  ]);
    const wallet = Array.from(bs58.decode(base58Key));
  console.log(`Byte Array: [${wallet}] \n\n`);
}

async function wallet_to_base58() {
   const { byteArray } = await inquirer.prompt([
    {
      type: 'input',
      name: 'byteArray',
      message: 'Enter comma-separated bytes:',
      validate: (input) => 
        /^(\d+,)*\d+$/.test(input) || 'Invalid format. Use commas between numbers'
    }
  ]);
  
  const bytes = byteArray.split(',').map(Number);
  const base58 = bs58.encode(Buffer.from(bytes));
  console.log(`Base58: ${base58}`);
}

async function main(){
await base58_to_wallet();

await wallet_to_base58();
}
main();

