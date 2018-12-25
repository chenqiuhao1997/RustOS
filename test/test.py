import os, sys, time, subprocess, signal

unit_test = []

realpath = sys.path[0]
os.system(realpath+'/clean_and_make.sh')
for unit_test_path in unit_test:
	if os.system('cd '+realpath+' && cd ..'+unit_test_path+' && cargo test'):
		print("unit test "+unit_test_path+" error")
		sys.exit(1)
		
# code for lab_test
labtest_checklist = ["kernel lab_test finished"]
qemu = subprocess.Popen(realpath+'/make_and_test.sh', shell=True, \
		stdin=subprocess.PIPE, \
		stdout=subprocess.PIPE, \
		stderr = subprocess.PIPE,\
		preexec_fn = os.setsid, close_fds=True \
		)
time.sleep(30)
os.kill(-qemu.pid, 9)
res = qemu.stdout.read()
for testcase in labtest_checklist:
	if res.find(testcase) < 0:
		print("%s not found!" % (testcase))
		sys.exit(4)
print("lab_test passed!")




