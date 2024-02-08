import { useState } from 'react';
import { Input, Select } from '@/recipes/input';

export function SelectFromOptions({
  initial,
  valueOptions,
  label,
  fieldId,
  allowCustom,
  allowNone,
}: {
  initial?: string;
  valueOptions: string[][];
  label: string;
  fieldId: string;
  allowCustom?: boolean;
  allowNone?: boolean;
}) {
  const selectedOption = initial && valueOptions.find(v => v[1] === initial);
  const [option, setOption] = useState(
    selectedOption
      ? selectedOption[0]
      : initial
        ? 'custom'
        : valueOptions[0][0],
  );

  const [customValue, setCustomValue] = useState(
    initial
      ? !valueOptions.map(v => v[1]).includes(initial)
        ? initial
        : ''
      : '',
  );

  return (
    <>
      <input
        type='hidden'
        name={fieldId}
        value={
          option === 'custom' && allowCustom && customValue
            ? customValue
            : option
        }
      />
      <label htmlFor={`_${fieldId}`}>{label}</label>
      <Select
        name={`_${fieldId}`}
        id={`_${fieldId}`}
        value={option}
        onChange={e => setOption(e.target.value)}>
        {allowNone && <option value=''>None</option>}
        {valueOptions.map(([value, label]) => (
          <option key={value} value={value}>
            {label}
          </option>
        ))}
        {allowCustom && <option value='custom'>Custom</option>}
      </Select>

      {option === 'custom' && allowCustom && (
        <Input
          type='text'
          name={`_${fieldId}`}
          id={`_${fieldId}`}
          placeholder={'Custom Value'}
          value={customValue}
          onChange={e => setCustomValue(e.target.value)}
        />
      )}
    </>
  );
}
