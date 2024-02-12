import { ReactNode, useState } from 'react';
import { Input, Select } from '@/recipes/input';

/**
 * SelectFromOptions method allows the user to select an option from a list of options
 *
 * @param {object} options - The options for the select element
 * @param {string} options.initial - The initial value of the select element
 * @param {string[][]} options.valueOptions - The list of options to choose from, each option is represented as an array with [value, label]
 * @param {string} options.label - The label for the select element
 * @param {string} options.fieldId - The id of the field
 * @param {boolean} options.allowCustom - Whether to allow a custom option to be entered
 * @param {boolean} options.allowNone - Whether to allow an empty option to be selected
 */
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
}): ReactNode {
  const selectedOption = initial && valueOptions.find(v => v[1] === initial);
  const [option, setOption] = useState(
    allowNone
      ? ''
      : selectedOption
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
