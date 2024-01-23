import { cva } from '@/styling/css';
import { styled } from '@/styling/jsx';

export const button = cva({
  base: {
    display: 'flex',
    alignItems: 'center',
    borderRadius: 'sm',
    py: 1,
    px: 4,
    gap: 2,
    cursor: 'pointer',
    fontWeight: 'semibold',
    transition: 'all 0.2s ease-in-out',
  },
  variants: {
    variant: {
      solid: {
        bg: 'primary',
        color: 'white',
        _hover: {
          bg: 'primary.hover',
          color: 'text.hover',
        },
      },
      outline: {
        outline: '2px solid token(colors.primary)',
        outlineOffset: '-1px',
        bg: 'transparent',
        color: 'primary',
        _hover: {
          bg: 'primary.hover',
          color: 'text.hover',
        },
      },
    },
    contentType: {
      icon: {
        px: 2,
        py: 1.5,
        rounded: 'md',
      },
    },
    pending: {
      true: {
        bg: 'primary',
        color: 'text.primary',
        _hover: {
          bg: 'primary',
          color: 'text.primary',
        },
        cursor: 'wait',
        filter: 'grayscale(80%)',
      },
    },
    disabled: {
      true: {
        opacity: 0.5,
        cursor: 'not-allowed',
      },
    },
  },
  defaultVariants: {
    variant: 'solid',
  },
});

const Button = styled('button', button);
export default Button;
