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

export default function EditProfileDialog({
  userId,
  profile,
}: {
  userId: number;
  profile: ProfileUpdatePayload;
}) {
  const [isOpen, setIsOpen] = useState(true);
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
          className={css({
            zIndex: 10,
            inset: 0,
          })}>
          <TransitionBackdrop />
          <TransitionChildWrapper>
            <Dialog.Panel
              className={css({
                pos: 'fixed',
                top: '2rem',
                right: '50%',
                transform: 'translateX(50%)',
                zIndex: 10,
                bg: 'surface',
                outline: '1px solid',
                oct: 'black/85',
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
              <form>
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
                  <EditField
                    label={'Bio'}
                    fieldId={'bio'}
                    value={profile.bio}
                  />
                </Stack>
                <HStack justify={'flex-end'} mt={1}>
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
}: {
  label: string;
  value?: string;
  fieldId: string;
}) {
  return (
    <Stack>
      <styled.label
        htmlFor={label}
        css={{
          mb: 1,
          color: 'text',
        }}>
        {label}
      </styled.label>
      <styled.input
        type={'text'}
        name={fieldId}
        id={label}
        defaultValue={value}
        css={{
          bg: 'surface',
          outline: '1px solid',
          oct: 'black/85',
          rounded: 'md',
          shadow: 'md',
          px: 2,
          py: 1,
        }}
      />
    </Stack>
  );
}
