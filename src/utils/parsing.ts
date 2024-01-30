import { z } from 'zod';

export const customDate = z.string().refine(
  value => {
    if (value.length === 0) return true;
    return !isNaN(Date.parse(value));
  },
  {
    message: 'Invalid date format',
  },
);