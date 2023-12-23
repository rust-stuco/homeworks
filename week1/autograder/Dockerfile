FROM gradescope/autograder-base

ADD source /autograder/source

RUN cp /autograder/source/run_autograder /autograder/run_autograder

# Ensure that scripts are Unix-friendly and executable
RUN dos2unix /autograder/run_autograder
RUN chmod +x /autograder/run_autograder

# Do whatever setup was needed in setup.sh, including installing apt packages
# Cleans up the apt cache afterwards in the same step to keep the image small
# RUN apt-get update && \
#     apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# Install Gradescope utilities for Python
RUN pip3 install -r /autograder/source/requirements.txt

# Set up nightly version of Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y

# Add .cargo/bin to PATH
ENV PATH="/root/.cargo/bin:${PATH}"