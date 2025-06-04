declare module 'libcaesium-wasm.js' {
  import { ILibcaesium } from '../index';

  const CaesiumWASM: () => Promise<ILibcaesium>;
  export default CaesiumWASM;
}
