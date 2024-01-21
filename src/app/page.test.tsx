import { render, screen } from '@testing-library/react';
import Page from './page';

describe('Start page', () => {
  it('Page renders', async () => {
    render(await Page());
    expect(screen.getByRole('heading').textContent).toBe('Not signed in');
  });
});
