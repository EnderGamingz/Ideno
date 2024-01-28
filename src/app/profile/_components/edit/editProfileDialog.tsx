'use client';

import { Fragment, useState } from 'react';
import Button from '@/recipes/button';
import Icon from '@/app/_components/icon';
import { Dialog, Transition } from '@headlessui/react';
import {
  TransitionBackdrop,
  TransitionChildWrapper,
} from '@/app/_components/TransitionWrapper';
import { css } from '@/styling/css';
import { DialogDescription, DialogTitle } from '@/app/_components/Dialog';
import { HStack, Stack, styled } from '@/styling/jsx';
import { ProfileUpdatePayload } from '@/types/profile';
import { stack } from '@/styling/patterns';
import profileUpdateAction from '@/app/profile/_components/edit/profileUpdateAction';
import { useRouter } from 'next/navigation';
import { SelectPronouns } from '@/app/profile/_components/edit/selectPronouns';

export const inputStyles = {
  outline: '1px solid',
  oct: 'primary/85',
  rounded: 'md',
  shadow: 'md',
  px: 2,
  py: 1,
  resize: 'none',
};

export default function EditProfileDialog({
  profile,
}: {
  profile: ProfileUpdatePayload;
}) {
  const router = useRouter();
  const [isOpen, setIsOpen] = useState(false);
  return (
    <>
      <Button
        variant={'outline'}
        contentType={'icon'}
        onClick={() => setIsOpen(true)}>
        <Icon.Edit />
      </Button>
      <Transition appear show={isOpen} as={Fragment}>
        <Dialog
          onClose={() => {}}
          className={stack({
            pos: 'fixed',
            inset: 0,
            zIndex: 10,
            justify: 'center',
            alignItems: 'center',
            p: 5,
          })}>
          <TransitionBackdrop />
          <TransitionChildWrapper>
            <Dialog.Panel
              className={css({
                zIndex: 10,
                bg: 'surface',
                rounded: 'md',
                shadow: 'md',
                px: 4,
                py: 3,
                maxW: 'lg',
                w: 'full',
              })}>
              <DialogTitle>Edit Profile Information</DialogTitle>
              <DialogDescription>
                Here you can update your profile details
              </DialogDescription>
              <form
                action={async data => {
                  await profileUpdateAction(data);
                  router.refresh();
                  setIsOpen(false);
                }}>
                <Stack>
                  <EditField
                    label={'First Name'}
                    fieldId={'first_name'}
                    value={profile.first_name}
                  />
                  <EditField
                    label={'Last Name'}
                    fieldId={'last_name'}
                    value={profile.last_name}
                  />
                  <EditField
                    label={'Headline'}
                    fieldId={'headline'}
                    value={profile.headline}
                  />
                  <Stack flexDirection={{ base: 'column', sm: 'row' }}>
                    <EditField
                      label={'City'}
                      fieldId={'city'}
                      value={profile.city}
                    />
                    <EditField
                      label={'Country'}
                      fieldId={'country'}
                      value={profile.country}
                    />
                  </Stack>
                  <SelectPronouns initial={profile.pronouns} />
                  <EditField
                    type={'textarea'}
                    label={'Bio'}
                    fieldId={'bio'}
                    value={profile.bio}
                  />
                </Stack>
                <HStack justify={'flex-end'} mt={5}>
                  <Button
                    variant={'secondary'}
                    type={'button'}
                    onClick={() => setIsOpen(false)}>
                    Cancel
                  </Button>
                  <Button type={'submit'}>
                    <Icon.Check /> Submit
                  </Button>
                </HStack>
              </form>
            </Dialog.Panel>
          </TransitionChildWrapper>
        </Dialog>
      </Transition>
    </>
  );
}

function EditField({
  label,
  value,
  fieldId,
  type,
}: {
  label: string;
  value?: string;
  fieldId: string;
  type?: string;
}) {
  return (
    <Stack gap={0}>
      <styled.label htmlFor={label} mb={1} color={'text'}>
        {label}
      </styled.label>
      {type === 'textarea' ? (
        <styled.textarea
          name={fieldId}
          id={label}
          defaultValue={value}
          rows={5}
          cols={30}
          css={inputStyles}
        />
      ) : (
        <styled.input
          type={'text'}
          name={fieldId}
          id={label}
          defaultValue={value}
          css={inputStyles}
        />
      )}
    </Stack>
  );
}
