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
      secondary: {
        bg: 'transparent',
        color: 'text',
        outline: '1px solid',
        oct: 'black/80',
        _hover: {
          bg: 'secondary.hover',
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
      iconRound: {
        p: 2,
        aspectRatio: 1,
        rounded: 'full',
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
    size: {
      small: {
        px: 2,
        py: 1,
        fontSize: '0.8rem',
      },
    },
  },
  defaultVariants: {
    variant: 'solid',
  },
});

const Button = styled('button', button);
export default Button;
