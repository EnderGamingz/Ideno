import { useState } from 'react';
import { styled } from '@/styling/jsx';
import { inputStyles } from '@/app/profile/_components/edit/editProfileDialog';

export function SelectPronouns({ initial }: { initial?: string }) {
  const pronounOptions = [
    ['he', 'he/him'],
    ['she', 'she/her'],
    ['they', 'they/them'],
  ];

  const selectedPronoun = initial && pronounOptions.find(v => v[1] === initial);
  const [pronouns, setPronouns] = useState(
    selectedPronoun ? selectedPronoun[0] : initial ? 'custom' : '',
  );

  const [customValue, setCustomValue] = useState(
    initial
      ? !pronounOptions.map(v => v[1]).includes(initial)
        ? initial
        : ''
      : '',
  );

  return (
    <>
      <input
        type='hidden'
        name='pronouns'
        value={pronouns === 'custom' && customValue ? customValue : pronouns}
      />
      <label htmlFor='_pronouns'>Pronouns</label>
      <styled.select
        css={inputStyles}
        name='_pronouns'
        id='_pronouns'
        value={pronouns}
        onChange={e => setPronouns(e.target.value)}>
        <option value=''>None</option>
        {pronounOptions.map(([value, label]) => (
          <option key={value} value={value}>
            {label}
          </option>
        ))}
        <option value='custom'>Custom</option>
      </styled.select>

      {pronouns === 'custom' && (
        <styled.input
          css={inputStyles}
          type='text'
          name='_pronouns'
          id='_pronouns'
          placeholder={'Custom Value'}
          value={customValue}
          onChange={e => setCustomValue(e.target.value)}
        />
      )}
    </>
  );
}
