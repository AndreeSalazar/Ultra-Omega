/* ═══════════════════════════════════════════════════════════════════════════
 * VULKAN TEXTURE - Carga y manejo de texturas
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: texture.cpp
 * Descripción: Cargar imágenes y crear texturas Vulkan
 * Hereda: buffers.cpp -> pipeline.cpp -> vulkan_types.h
 * ═══════════════════════════════════════════════════════════════════════════
 */

#include "vulkan_types.h"
#include <iostream>
#include <stdexcept>
#include <cstring>

/* Declaraciones externas */
extern uint32_t findMemoryType(VulkanContext& ctx, uint32_t typeFilter, 
    VkMemoryPropertyFlags properties);
extern VkImageView createImageView(VulkanContext& ctx, VkImage image, VkFormat format, 
    VkImageAspectFlags aspectFlags);

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR IMAGEN
 * ═══════════════════════════════════════════════════════════════════════════
 */

void createImage(VulkanContext& ctx, uint32_t width, uint32_t height, VkFormat format,
    VkImageTiling tiling, VkImageUsageFlags usage, VkMemoryPropertyFlags properties,
    VkImage& image, VkDeviceMemory& imageMemory) {
    
    VkImageCreateInfo imageInfo{};
    imageInfo.sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO;
    imageInfo.imageType = VK_IMAGE_TYPE_2D;
    imageInfo.extent.width = width;
    imageInfo.extent.height = height;
    imageInfo.extent.depth = 1;
    imageInfo.mipLevels = 1;
    imageInfo.arrayLayers = 1;
    imageInfo.format = format;
    imageInfo.tiling = tiling;
    imageInfo.initialLayout = VK_IMAGE_LAYOUT_UNDEFINED;
    imageInfo.usage = usage;
    imageInfo.samples = VK_SAMPLE_COUNT_1_BIT;
    imageInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;

    if (vkCreateImage(ctx.device, &imageInfo, nullptr, &image) != VK_SUCCESS) {
        throw std::runtime_error("Error al crear imagen!");
    }

    VkMemoryRequirements memRequirements;
    vkGetImageMemoryRequirements(ctx.device, image, &memRequirements);

    VkMemoryAllocateInfo allocInfo{};
    allocInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    allocInfo.allocationSize = memRequirements.size;
    allocInfo.memoryTypeIndex = findMemoryType(ctx, memRequirements.memoryTypeBits, 
        properties);

    if (vkAllocateMemory(ctx.device, &allocInfo, nullptr, &imageMemory) != VK_SUCCESS) {
        throw std::runtime_error("Error al asignar memoria de imagen!");
    }

    vkBindImageMemory(ctx.device, image, imageMemory, 0);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * TRANSICIÓN DE LAYOUT DE IMAGEN
 * ═══════════════════════════════════════════════════════════════════════════
 */

void transitionImageLayout(VulkanContext& ctx, VkImage image, VkFormat format,
    VkImageLayout oldLayout, VkImageLayout newLayout) {
    
    VkCommandBufferAllocateInfo allocInfo{};
    allocInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
    allocInfo.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY;
    allocInfo.commandPool = ctx.commandPool;
    allocInfo.commandBufferCount = 1;

    VkCommandBuffer commandBuffer;
    vkAllocateCommandBuffers(ctx.device, &allocInfo, &commandBuffer);

    VkCommandBufferBeginInfo beginInfo{};
    beginInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
    beginInfo.flags = VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT;

    vkBeginCommandBuffer(commandBuffer, &beginInfo);

    VkImageMemoryBarrier barrier{};
    barrier.sType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER;
    barrier.oldLayout = oldLayout;
    barrier.newLayout = newLayout;
    barrier.srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
    barrier.dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED;
    barrier.image = image;
    barrier.subresourceRange.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
    barrier.subresourceRange.baseMipLevel = 0;
    barrier.subresourceRange.levelCount = 1;
    barrier.subresourceRange.baseArrayLayer = 0;
    barrier.subresourceRange.layerCount = 1;

    VkPipelineStageFlags sourceStage;
    VkPipelineStageFlags destinationStage;

    if (oldLayout == VK_IMAGE_LAYOUT_UNDEFINED && 
        newLayout == VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL) {
        barrier.srcAccessMask = 0;
        barrier.dstAccessMask = VK_ACCESS_TRANSFER_WRITE_BIT;
        sourceStage = VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT;
        destinationStage = VK_PIPELINE_STAGE_TRANSFER_BIT;
    } else if (oldLayout == VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL && 
               newLayout == VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL) {
        barrier.srcAccessMask = VK_ACCESS_TRANSFER_WRITE_BIT;
        barrier.dstAccessMask = VK_ACCESS_SHADER_READ_BIT;
        sourceStage = VK_PIPELINE_STAGE_TRANSFER_BIT;
        destinationStage = VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT;
    } else {
        throw std::invalid_argument("Transición de layout no soportada!");
    }

    vkCmdPipelineBarrier(commandBuffer, sourceStage, destinationStage,
        0, 0, nullptr, 0, nullptr, 1, &barrier);

    vkEndCommandBuffer(commandBuffer);

    VkSubmitInfo submitInfo{};
    submitInfo.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO;
    submitInfo.commandBufferCount = 1;
    submitInfo.pCommandBuffers = &commandBuffer;

    vkQueueSubmit(ctx.graphicsQueue, 1, &submitInfo, VK_NULL_HANDLE);
    vkQueueWaitIdle(ctx.graphicsQueue);

    vkFreeCommandBuffers(ctx.device, ctx.commandPool, 1, &commandBuffer);
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR TEXTURA PROCEDURAL (Sin dependencia de stb_image)
 * ═══════════════════════════════════════════════════════════════════════════
 */

void createProceduralTexture(VulkanContext& ctx, uint32_t width, uint32_t height) {
    // Crear datos de textura procedural (tablero de ajedrez)
    std::vector<uint8_t> pixels(width * height * 4);
    
    for (uint32_t y = 0; y < height; y++) {
        for (uint32_t x = 0; x < width; x++) {
            uint32_t idx = (y * width + x) * 4;
            bool isWhite = ((x / 8) + (y / 8)) % 2 == 0;
            
            pixels[idx + 0] = isWhite ? 255 : 50;   // R
            pixels[idx + 1] = isWhite ? 255 : 50;   // G
            pixels[idx + 2] = isWhite ? 255 : 50;   // B
            pixels[idx + 3] = 255;                   // A
        }
    }

    VkDeviceSize imageSize = width * height * 4;

    // Staging buffer
    VkBuffer stagingBuffer;
    VkDeviceMemory stagingBufferMemory;
    
    VkBufferCreateInfo bufferInfo{};
    bufferInfo.sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO;
    bufferInfo.size = imageSize;
    bufferInfo.usage = VK_BUFFER_USAGE_TRANSFER_SRC_BIT;
    bufferInfo.sharingMode = VK_SHARING_MODE_EXCLUSIVE;

    vkCreateBuffer(ctx.device, &bufferInfo, nullptr, &stagingBuffer);

    VkMemoryRequirements memRequirements;
    vkGetBufferMemoryRequirements(ctx.device, stagingBuffer, &memRequirements);

    VkMemoryAllocateInfo allocInfo{};
    allocInfo.sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO;
    allocInfo.allocationSize = memRequirements.size;
    allocInfo.memoryTypeIndex = findMemoryType(ctx, memRequirements.memoryTypeBits,
        VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);

    vkAllocateMemory(ctx.device, &allocInfo, nullptr, &stagingBufferMemory);
    vkBindBufferMemory(ctx.device, stagingBuffer, stagingBufferMemory, 0);

    // Copiar datos a staging
    void* data;
    vkMapMemory(ctx.device, stagingBufferMemory, 0, imageSize, 0, &data);
    memcpy(data, pixels.data(), static_cast<size_t>(imageSize));
    vkUnmapMemory(ctx.device, stagingBufferMemory);

    // Crear imagen
    createImage(ctx, width, height, VK_FORMAT_R8G8B8A8_SRGB, VK_IMAGE_TILING_OPTIMAL,
        VK_IMAGE_USAGE_TRANSFER_DST_BIT | VK_IMAGE_USAGE_SAMPLED_BIT,
        VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT, ctx.textureImage, ctx.textureImageMemory);

    // Transiciones y copia
    transitionImageLayout(ctx, ctx.textureImage, VK_FORMAT_R8G8B8A8_SRGB,
        VK_IMAGE_LAYOUT_UNDEFINED, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL);

    // Copiar buffer a imagen
    VkCommandBufferAllocateInfo cmdAllocInfo{};
    cmdAllocInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO;
    cmdAllocInfo.level = VK_COMMAND_BUFFER_LEVEL_PRIMARY;
    cmdAllocInfo.commandPool = ctx.commandPool;
    cmdAllocInfo.commandBufferCount = 1;

    VkCommandBuffer commandBuffer;
    vkAllocateCommandBuffers(ctx.device, &cmdAllocInfo, &commandBuffer);

    VkCommandBufferBeginInfo beginInfo{};
    beginInfo.sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO;
    beginInfo.flags = VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT;
    vkBeginCommandBuffer(commandBuffer, &beginInfo);

    VkBufferImageCopy region{};
    region.bufferOffset = 0;
    region.bufferRowLength = 0;
    region.bufferImageHeight = 0;
    region.imageSubresource.aspectMask = VK_IMAGE_ASPECT_COLOR_BIT;
    region.imageSubresource.mipLevel = 0;
    region.imageSubresource.baseArrayLayer = 0;
    region.imageSubresource.layerCount = 1;
    region.imageOffset = {0, 0, 0};
    region.imageExtent = {width, height, 1};

    vkCmdCopyBufferToImage(commandBuffer, stagingBuffer, ctx.textureImage,
        VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 1, &region);

    vkEndCommandBuffer(commandBuffer);

    VkSubmitInfo submitInfo{};
    submitInfo.sType = VK_STRUCTURE_TYPE_SUBMIT_INFO;
    submitInfo.commandBufferCount = 1;
    submitInfo.pCommandBuffers = &commandBuffer;

    vkQueueSubmit(ctx.graphicsQueue, 1, &submitInfo, VK_NULL_HANDLE);
    vkQueueWaitIdle(ctx.graphicsQueue);

    vkFreeCommandBuffers(ctx.device, ctx.commandPool, 1, &commandBuffer);

    transitionImageLayout(ctx, ctx.textureImage, VK_FORMAT_R8G8B8A8_SRGB,
        VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL);

    // Cleanup staging
    vkDestroyBuffer(ctx.device, stagingBuffer, nullptr);
    vkFreeMemory(ctx.device, stagingBufferMemory, nullptr);

    std::cout << "[OK] Textura procedural creada (" << width << "x" << height << ")" 
              << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR TEXTURE IMAGE VIEW
 * ═══════════════════════════════════════════════════════════════════════════
 */

void createTextureImageView(VulkanContext& ctx) {
    ctx.textureImageView = createImageView(ctx, ctx.textureImage, 
        VK_FORMAT_R8G8B8A8_SRGB, VK_IMAGE_ASPECT_COLOR_BIT);
    std::cout << "[OK] Texture image view creada" << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CREAR SAMPLER
 * ═══════════════════════════════════════════════════════════════════════════
 */

void createTextureSampler(VulkanContext& ctx) {
    VkPhysicalDeviceProperties properties{};
    vkGetPhysicalDeviceProperties(ctx.physicalDevice, &properties);

    VkSamplerCreateInfo samplerInfo{};
    samplerInfo.sType = VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO;
    samplerInfo.magFilter = VK_FILTER_LINEAR;
    samplerInfo.minFilter = VK_FILTER_LINEAR;
    samplerInfo.addressModeU = VK_SAMPLER_ADDRESS_MODE_REPEAT;
    samplerInfo.addressModeV = VK_SAMPLER_ADDRESS_MODE_REPEAT;
    samplerInfo.addressModeW = VK_SAMPLER_ADDRESS_MODE_REPEAT;
    samplerInfo.anisotropyEnable = VK_TRUE;
    samplerInfo.maxAnisotropy = properties.limits.maxSamplerAnisotropy;
    samplerInfo.borderColor = VK_BORDER_COLOR_INT_OPAQUE_BLACK;
    samplerInfo.unnormalizedCoordinates = VK_FALSE;
    samplerInfo.compareEnable = VK_FALSE;
    samplerInfo.compareOp = VK_COMPARE_OP_ALWAYS;
    samplerInfo.mipmapMode = VK_SAMPLER_MIPMAP_MODE_LINEAR;

    if (vkCreateSampler(ctx.device, &samplerInfo, nullptr, 
        &ctx.textureSampler) != VK_SUCCESS) {
        throw std::runtime_error("Error al crear sampler de textura!");
    }

    std::cout << "[OK] Texture sampler creado" << std::endl;
}

/* ═══════════════════════════════════════════════════════════════════════════
 * CLEANUP
 * ═══════════════════════════════════════════════════════════════════════════
 */

void cleanupTexture(VulkanContext& ctx) {
    vkDestroySampler(ctx.device, ctx.textureSampler, nullptr);
    vkDestroyImageView(ctx.device, ctx.textureImageView, nullptr);
    vkDestroyImage(ctx.device, ctx.textureImage, nullptr);
    vkFreeMemory(ctx.device, ctx.textureImageMemory, nullptr);
    std::cout << "[OK] Textura destruida" << std::endl;
}

