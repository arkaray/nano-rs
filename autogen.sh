#!/bin/sh
# Generate configure & friends for GIT users.

gnulib_url="git://git.sv.gnu.org/gnulib.git"
gnulib_hash="ab02af9d47248a1d8b4ade4e8ab2c927d0543ea1"

modules="
	futimens
	getdelim
	getline
	getopt-gnu
	glob
	isblank
	iswblank
	lstat
	nl_langinfo
	regex
	sigaction
	snprintf-posix
	stdarg
	strcase
	strcasestr-simple
	strnlen
	sys_wait
	vsnprintf-posix
	wchar
	wctype-h
	wcwidth
"

# Make sure the local gnulib git repo is up-to-date.
if [ ! -d "gnulib" ]; then
	git clone --depth=123 ${gnulib_url}
fi
cd gnulib >/dev/null || exit 1
curr_hash=$(git log -1 --format=%H)
if [ "${gnulib_hash}" != "${curr_hash}" ]; then
	git pull
	git checkout -f ${gnulib_hash}
fi
cd .. >/dev/null || exit 1

rm -rf lib
./gnulib/gnulib-tool \
	--import \
	${modules}

autoreconf -f -i -s
