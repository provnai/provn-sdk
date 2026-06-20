jest.mock('../wasm/provn_sdk_wasm.js', () => require('./__mocks__/provn_sdk_wasm.js'));

import { initSDK } from '../src/index';

beforeAll(async () => {
  await initSDK();
});
