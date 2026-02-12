module.exports = {
  parser: '@typescript-eslint/parser',
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
  ],
  parserOptions: {
    ecmaVersion: 2020,
    sourceType: 'module',
    project: './tsconfig.json',
  },
  plugins: ['@typescript-eslint'],
  rules: {
    // Allow any for WASM integration
    '@typescript-eslint/no-explicit-any': 'off',
    // Allow require() for Node.js compatibility
    '@typescript-eslint/no-var-requires': 'off',
    // Consistent return type
    '@typescript-eslint/explicit-function-return-type': 'off',
    // Console usage is fine for SDK
    'no-console': 'off',
    // Allow unused vars in test files
    '@typescript-eslint/no-unused-vars': ['error', { 
      'argsIgnorePattern': '^_',
      'varsIgnorePattern': '^_' 
    }],
  },
  ignorePatterns: [
    'dist/',
    'node_modules/',
    'wasm/',
    '*.js',
    'jest.config.js',
  ],
};
