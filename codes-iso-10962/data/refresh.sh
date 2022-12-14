#!/usr/bin/env sh

if [[ ! -x "$(command -v pip3)" ]] ; then
    python3 -m pip install --upgrade pip
else
    echo "pip installed"
fi

if [[ ! -x "$(command -v xlsx2csv)" ]] ; then
    pip3 install xlsx2csv
else
    echo "xlsx2csv installed"
fi

PY_USER_BASE$(python3 -c "exec(\"import site\nprint(site.USER_BASE)\")")
PATH=$PATH:$PY_USER_BASE/bin

curl -o ./cfi-current.xlsx "https://www.six-group.com/dam/download/financial-information/data-center/cfi/cfi-20210507-current.xlsx"

xlsx2csv -a ./cfi-current.xlsx csv-sheets
