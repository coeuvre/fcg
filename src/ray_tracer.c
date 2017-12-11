#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb_image_write.h"

#define RGBA(r, g, b, a) \
  ((((r)&0xFF) << 24) | (((g)&0xFF) << 16) | (((b)&0xFF) << 8) | ((a)&0xFF))

typedef struct ImageBuffer ImageBuffer;
struct ImageBuffer {
  int width;
  int height;
  uint32_t *data;
};

static ImageBuffer *createImageBuffer(int width, int height) {
  ImageBuffer *ib = malloc(sizeof(ImageBuffer));
  ib->width = width;
  ib->height = height;
  ib->data = malloc(sizeof(uint32_t) * width * height);
  return ib;
}

static void destroyImageBuffer(ImageBuffer **ref) {
  ImageBuffer *ib = *ref;
  free(ib->data);
  free(ib);
  *ref = 0;
}

static void writeImageBufferToPng(ImageBuffer *ib, const char *filename) {
  stbi_write_png(filename, ib->width, ib->height, 4, ib->data,
                 sizeof(uint32_t) * ib->width);
}

static void setImageBufferPixel(ImageBuffer *ib, int x, int y, uint32_t pixel) {
  if (x < 0 || x >= ib->width) {
    return;
  }

  if (y < 0 || y >= ib->height) {
    return;
  }

  uint32_t *dst = ((uint32_t *)ib->data) + y * ib->width + x;
  *dst = pixel;
}

static void clearImageBuffer(ImageBuffer *ib, uint32_t pixel) {
  for (int y = 0; y < ib->height; ++y) {
    for (int x = 0; x < ib->width; ++x) {
      setImageBufferPixel(ib, x, y, pixel);
    }
  }
}

int main(int argc, const char **argv) {
  ImageBuffer *ib = createImageBuffer(1280, 720);

  clearImageBuffer(ib, RGBA(255, 255, 255, 255));

  writeImageBufferToPng(ib, "output.png");

  destroyImageBuffer(&ib);

  return 0;
}