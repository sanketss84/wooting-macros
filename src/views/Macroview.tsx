import { VStack, HStack } from '@chakra-ui/react'
import EditArea from '../components/macroview/rightPanel/EditArea'
import SelectElementArea from '../components/macroview/leftPanel/SelectElementArea'
import SequencingArea from '../components/macroview/centerPanel/SequencingArea'
import Header from '../components/macroview/topArea/Header'

type Props = {
  isEditing: boolean
}

export default function Macroview({ isEditing }: Props) {
  return (
    <VStack h="full" spacing="0px" overflow="hidden">
      {/** Top Header */}
      <Header isEditing={isEditing} />
      <HStack
        w="full"
        h={{
          base: 'calc(100% - 80px)',
          md: 'calc(100% - 100px)',
          xl: 'calc(100% - 120px)'
        }}
        spacing="0"
      >
        {/** Bottom Panels */}
        <SelectElementArea />
        <SequencingArea />
        <EditArea />
      </HStack>
    </VStack>
  )
}
