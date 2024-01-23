import { defineConfig } from '@pandacss/dev';
import { NestedCssProperties } from '@/styling/types';

function transform(returnValue: string, value: any, { token }: { token: any }) {
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
}

let colors = {
  // Core
  primary: {
    DEFAULT: { value: '#60c7c7' },
    muted: { value: '#4e929b' },
    hover: { value: '#95dce8' },
  },
  secondary: {
    DEFAULT: { value: '#7e9dec' },
    muted: { value: '#7588b9' },
    hover: { value: '#a9bdec' },
  },
  tertiary: {
    DEFAULT: { value: '#AF67DD' },
    muted: { value: '#5b4477' },
    hover: { value: '#9E5FD0' },
  },

  text: {
    hover: { value: '#152f2f' },
  },

  // Surface and backgrounds
  background: { value: '#f7fdfd' },
  surface: {
    DEFAULT: { value: '#3A3B5F' },
    hover: { value: '#4e4e6c' },
    muted: { value: '#2A2B4F' },
  },
  surfaceAlt: {
    DEFAULT: { value: '#212121' },
    hover: { value: '#2e2e2e' },
    muted: { value: '#1e1e1e' },
  },
  input: {
    DEFAULT: { value: '#e5f3f3' },
    muted: { value: '#c2d0d0' },
    hover: { value: '#eaf3f3' },
  },

  // Feedback colors
  success: {
    DEFAULT: { value: '#77d377' },
    muted: { value: '#5fa85f' },
    hover: { value: '#6abf71' },
  },
  error: {
    DEFAULT: { value: '#d37777' },
    muted: { value: '#a55f5f' },
    hover: { value: '#c4666b' },
  },
  warning: {
    DEFAULT: { value: '#FFA500' },
    muted: { value: '#cc8400' },
    hover: { value: '#ffb734' },
  },
};

export default defineConfig({
  preflight: true,
  include: [
    './src/components/**/*.{ts,tsx}',
    './src/app/**/*.{ts,tsx}',
    './src/recipes/**/*.{ts,tsx}',
  ],
  exclude: [],
  globalCss: {
    body: {
      bg: 'background',
    },
  },

  theme: {
    extend: {
      tokens: {
        colors: colors,
      },
    },
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
    borderColorTransparentize: {
      shorthand: ['bct'],
      property: 'borderColor',
      className: 'transparentize_bc',
      transform: (x, y) =>
        transform('borderColor', x, y) as unknown as
          | NestedCssProperties
          | undefined,
    },
  },
  outdir: 'styled-system',
  jsxFramework: 'react',
  jsxStyleProps: 'all',
});
