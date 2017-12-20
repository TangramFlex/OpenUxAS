FROM resin/armv7hf-debian

RUN [ "cross-build-start" ]

RUN apt-get update

# install all the stuffs
RUN apt-get install -y git gcc g++ curl libglu1-mesa-dev ant \
  uuid-dev libboost-filesystem-dev libboost-regex-dev \
  libboost-system-dev unzip python python3 pkg-config
RUN echo deb http://ftp.debian.org/debian jessie-backports main >>/etc/apt/sources.list
RUN apt-get update
RUN apt-get -t jessie-backports install -y openjdk-8-jdk
RUN update-java-alternatives --set /usr/lib/jvm/java-1.8.0-openjdk-armhf

# download and build meson
RUN curl -L -s https://github.com/mesonbuild/meson/archive/0.42.1.zip -o /opt/meson-linux.zip
RUN cd /opt/ && unzip meson-linux.zip
RUN cd /opt/meson-0.42.1 && python3 setup.py install

# download and build ninja
RUN curl -L -s https://github.com/ninja-build/ninja/archive/v1.7.2.zip -o /opt/ninja.zip
RUN cd /opt/ && unzip ninja.zip
RUN cd /opt/ninja-1.7.2 && ./configure.py --bootstrap && mv ./ninja /bin/ninja

# get and run the Lmcp binding generator
RUN git clone https://github.com/afrl-rq/LmcpGen.git /opt/LmcpGen
RUN cd /opt/LmcpGen && ant jar

# and the actual build
RUN git clone -b docker https://github.com/GaloisInc/OpenUxAS.git /opt/uxas
RUN cd /opt/uxas && sh ./RunLmcpGen.sh
RUN cd /opt/uxas && ./prepare
RUN cd /opt/uxas && meson build
RUN cd /opt/uxas && ninja -C build uxas

RUN [ "cross-build-end" ]
