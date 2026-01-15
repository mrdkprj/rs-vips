FROM rust:1.92.0-slim

ENV VIPS_VERSION=8.17.3

RUN apt-get update -qq && apt-get install -qq -y \
  build-essential \
  wget \
  pkg-config \
  libclang-dev \
  meson \
  ninja-build \
  libfftw3-dev \
  libcgif-dev \
  libexpat-dev \
  gobject-introspection \
  liblcms2-dev \
  libexif-dev \
  libheif-dev \
  libimagequant-dev \
  libpng-dev \
  librsvg2-dev \
  libwebp-dev \
  libtiff-dev \
  libarchive-dev \
  libcfitsio-dev \
  libopenexr-dev \
  libjxl-dev \
  libopenjp2-7-dev \
  libpoppler-glib-dev \
  libmatio-dev \
  libopenslide-dev \
  && rm -rf /var/lib/apt/lists/*

RUN wget -q https://github.com/libvips/libvips/releases/download/v${VIPS_VERSION}/vips-${VIPS_VERSION}.tar.xz \
  && mkdir vips \
  && tar xJf vips-${VIPS_VERSION}.tar.xz -C vips --strip-components 1 \
  && cd /vips \
  && meson setup build \
  && cd /vips/build \
  && meson compile >/dev/null \
  && meson install > /dev/null \
  && ldconfig /etc/ld.so.conf.d \
  && rm -rf vips \
  && rm -f vips-${VIPS_VERSION}.tar.xz \
  && rustup component add rustfmt
