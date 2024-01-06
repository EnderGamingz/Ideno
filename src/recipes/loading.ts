import { cva } from '@/styling/css';
import { styled } from '@/styling/jsx';

export const loading = cva({
  base: {
    zIndex: 1,
    w: '35px',
    aspectRatio: '1/1',
    borderBottomColor: 'transparent!',
    rounded: 'full',
    display: 'inline-block',
    boxSizing: 'border-box',
    animation: 'spin 0.8s linear infinite',
  },
  variants: {
    color: {
      primary: {
        border: '3px solid token(colors.primary)',
      },
      secondary: {
        border: '3px solid token(colors.secondary)',
      },
    },
  },
  defaultVariants: {
    color: 'primary',
  },
});

const Loading = styled('div', loading);
export default Loading;
