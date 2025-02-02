import {
  AccordionButton,
  AccordionIcon,
  AccordionItem,
  AccordionPanel,
  AspectRatio,
  Flex,
  SimpleGrid
} from '@chakra-ui/react'
import { SystemEventInfo } from '../../../../constants/SystemEventMap'
import { SystemIcon } from '../../../icons'
import SelectElementButton from '../SelectElementButton'

interface Props {
  elementsToRender: SystemEventInfo[]
}

export default function SystemEventsSection({ elementsToRender }: Props) {
  return (
    <AccordionItem>
      <h2>
        <AccordionButton>
          <Flex
            flex="1"
            textAlign="left"
            fontWeight="semibold"
            alignItems="center"
            gap={2}
          >
            <SystemIcon />
            System Events
          </Flex>
          <AccordionIcon boxSize={6} />
        </AccordionButton>
      </h2>
      <AccordionPanel>
        <SimpleGrid
          h="fit"
          columns={{
            base: 2,
            md: 3,
            xl: 4
          }}
          px={4}
          spacing={2}
        >
          {elementsToRender.map((info: SystemEventInfo) => (
            <AspectRatio ratio={2 / 0.75} key={info.displayString}>
              <SelectElementButton
              key={info.displayString}
                nameText={info.displayString}
                descText={info.description}
                properties={{
                  type: 'SystemEventAction',
                  data: info.defaultData
                }}
              />
            </AspectRatio>
          ))}
        </SimpleGrid>
      </AccordionPanel>
    </AccordionItem>
  )
}
