import { Box, Image } from '@mantine/core'
import React from 'react'

interface IFloatingImage{
    src: string
    top?: string
    right?: string
    bottom?: string
    left?: string
    width: string
}

const FloatingImage = (props: IFloatingImage) => {
    const {src, top, right, bottom, left, width} = props
  return (
    <Box style={{
        position: 'fixed',
        top: top ? top : 'auto',
        right: right ? right : 'auto',
        bottom: bottom ? bottom : 'auto',
        left: left ? left : 'auto',
        zIndex: -1
    }}>
        <Image w={width} src={src} />
    </Box>
  )
}

export default FloatingImage