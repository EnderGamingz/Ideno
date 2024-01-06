import { defineConfig } from '@pandacss/dev';
import { NestedCssProperties } from '@/styling/types';

const transform = (
  returnValue: string,
  value: any,
  { token }: { token: any },
) => {
  const lastIndex = value?.lastIndexOf('/');
  if (!lastIndex || typeof value?.substring !== 'function') {
    return {};
  }

  const color = value?.substring(0, lastIndex);
  if (!color) {
    return {};
  }

  const calculateValue = (tokenPath: string, property: any) => {
    const tokenValue = token(tokenPath);
    return tokenValue ? +tokenValue * 100 : `${property}%`;
  };

  const colorValue = token(`colors.${color}`);
  const amount = value.split('/').at(-1);
  const amountValue = calculateValue(`opacity.${amount}`, amount);

  return {
    [returnValue]: `color-mix(in srgb, transparent ${amountValue}, ${colorValue}) !important`,
  };
};

export default defineConfig({
  preflight: true,
  include: [
    './src/components/**/*.{ts,tsx}',
    './src/app/**/*.{ts,tsx}',
    './src/recipes/**/*.{ts,tsx}',
  ],
  exclude: [],

  theme: {
    extend: {},
  },

  utilities: {
    backgroundColorTransparentize: {
      shorthand: ['bgct', 'bgt'],
      property: 'backgroundColor',
      className: 'transparentize_bgc',
      transform: (x, y) =>
        transform('backgroundColor', x, y) as unknown as
          | NestedCssProperties
          | undefined,
    },
    colorTransparentize: {
      shorthand: ['ct'],
      property: 'color',
      className: 'transparentize_c',
      transform: (x, y) =>
        transform('color', x, y) as unknown as NestedCssProperties | undefined,
    },
    outlineColorTransparentize: {
      shorthand: ['oct'],
      property: 'outlineColor',
      className: 'transparentize_ot',
      transform: (x, y) =>
        transform('outlineColor', x, y) as unknown as
          | NestedCssProperties
          | undefined,
    },
  },
  outdir: 'styled-system',
  jsxFramework: 'react',
  jsxStyleProps: 'all',
});
