import os, sys, time, subprocess, signal

checklist = {
	'main':{'time':5, 'target': '', 'rule':[
		"idle proc start correctly",
		"init proc start correctly",
		"init proc exited correctly",

	]},
}

unit_test = ['/crate/memory']

realpath = sys.path[0]
os.system(realpath+'/clean_and_make.sh')


for unit_test_path in unit_test:
	if os.system('cd '+realpath+' && cd ..'+unit_test_path+' && cargo test'):
		print("unit test "+unit_test_path+" error")
		sys.exit(1)



os.system(realpath+'/clean_and_make.sh')
for testcase in checklist.keys():
	target = testcase
	if checklist[testcase].has_key('target'):
		target = ' ' + checklist[testcase]['target']
	qemu = subprocess.Popen(realpath+'/make_and_run.sh'+target, shell=True, \
		stdin=subprocess.PIPE, \
		stdout=subprocess.PIPE, \
		stderr = subprocess.PIPE,\
		preexec_fn = os.setsid, close_fds=True \
		)

	time.sleep(5)

	time.sleep(checklist[testcase]['time'])
	os.kill(-qemu.pid, 9)
	res = qemu.stdout.read()
	point = 1
	for output in checklist[testcase]['rule']:
		if res.find(output) < 0:
			point = 0
			print(output)
			break
	if point == 1:
		print(testcase+": OK")
	else:
		print(testcase+": ERROR")
		print(res)
		sys.exit(3)

