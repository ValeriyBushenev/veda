package main

import (
	"github.com/gorilla/websocket"
	"log"
	"strconv"
	"strings"
	"time"
)

type ccusConn struct {
	ready       bool
	ws          *websocket.Conn
	chid        string
	count_2_uid map[string]int
	cc          chan updateInfo
}

func (pc *ccusConn) get_counter_4_uid(uid string) int {
	pc.cc <- updateInfo{uid, 0, -1}
	g_info := <-pc.cc
	//log.Printf("ws[%s]:get_counter_4_uid[%s] %d", pc.ws.RemoteAddr(), uid, g_info.update_counter)
	return g_info.update_counter
}

func (pc *ccusConn) get_last_opid() int {
	pc.cc <- updateInfo{"", -1, 0}
	g_info := <-pc.cc
	//log.Printf("ws[%s]:get_last_opid %d", pc.ws.RemoteAddr(), g_info.opid)
	return g_info.opid
}

func (pc *ccusConn) get_list_of_subscribe() string {
	res := ""

	for i_uid, i_count := range pc.count_2_uid {

		g_count := pc.get_counter_4_uid(i_uid)

		if g_count > i_count {
			i_count := g_count
			if g_count > i_count {
				i_count = g_count
				pc.count_2_uid[i_uid] = i_count
			}

			if len(res) == 0 {
				res = res + i_uid + "=" + strconv.Itoa(i_count)
			} else {
				res = res + "," + i_uid + "=" + strconv.Itoa(i_count)
			}
			pc.count_2_uid[i_uid] = i_count
		}
	}
	return res
}

func (pc *ccusConn) get_list_of_changes() string {
	res := ""

	for i_uid, i_count := range pc.count_2_uid {

		g_count := pc.get_counter_4_uid(i_uid)

		if g_count > i_count {
			i_count := g_count

			if len(res) == 0 {
				res = res + i_uid + "=" + strconv.Itoa(i_count)
			} else {
				res = res + "," + i_uid + "=" + strconv.Itoa(i_count)
			}
			pc.count_2_uid[i_uid] = i_count
		}
	}
	return res
}

func timer1(cc_prepare chan string) {
	for {
		cc_prepare <- "T"
		time.Sleep(1000 * time.Millisecond)
	}
}

func (pc *ccusConn) preparer(cc_prepare chan string) {
	last_check_opid := 0
	pc.count_2_uid = make(map[string]int)

	for {
		msg := <-cc_prepare

		if len(msg) > 1 {
			log.Printf("ws[%s]:receive msg %s", pc.ws.RemoteAddr(), msg)

			if pc.ready == false {
				kv := strings.Split(msg, "=")

				if len(kv) == 2 {
					if strings.Compare(kv[0], "ccus") == 0 {
						pc.chid = kv[1]
						log.Printf("ws[%s]:init chanell %s", pc.ws.RemoteAddr(), pc.chid)
					}
				}
				pc.ready = true
			} else {
				if msg[0] == '#' {
					msg_parts := strings.Split(msg, ";")
					if len(msg_parts) == 3 {

						uid := msg_parts[0][1:len(msg_parts[0])]
						update_counter, err := strconv.Atoi(msg_parts[1])
						if err != nil {
							continue
						}
						opid, err := strconv.Atoi(msg_parts[2])
						if err != nil {
							continue
						}
						//					log.Printf("ws[%s] @1")
						ni := updateInfo{uid, opid, update_counter}
						//					log.Printf("ws[%s] @2 ni=%s", ni)
						pc.cc <- ni
						//					log.Printf("ws[%s] @3")

					} else {
						log.Printf("ws[%s][#] send=%s", pc.ws.RemoteAddr(), "Err:invalid message")
						pc.ws.WriteMessage(websocket.TextMessage, []byte("Err:invalid message"))
					}
					continue

				} else if msg[0] == '=' {

					res := pc.get_list_of_subscribe()
					pc.ws.WriteMessage(websocket.TextMessage, []byte("="+res))

				} else if len(msg) == 2 && msg[0] == '-' && msg[1] == '*' {

					pc.count_2_uid = make(map[string]int)

				} else if len(msg) > 3 {
					msg_parts := strings.Split(msg, ",")

					for _, element := range msg_parts {
						expr := strings.Split(element, "=")

						uid_info := ""

						if len(expr) > 0 {
							uid_info = expr[0]
						}

						if len(expr) == 2 {
							if len(uid_info) > 2 {
								uid := uid_info[1:len(uid_info)]
								if uid_info[0] == '+' {
									uid_counter, err := strconv.Atoi(expr[1])
									if err != nil {
										continue
									}
									//log.Printf("ws[%s]:receive uid=%s uid_counter=%d", pc.ws.RemoteAddr(), uid, uid_counter)
									pc.count_2_uid[uid] = uid_counter
									g_count := pc.get_counter_4_uid(uid)
									if uid_counter < g_count {
										res := pc.get_list_of_subscribe()
										log.Printf("ws[%s][+] send=%s", pc.ws.RemoteAddr(), res)
										pc.ws.WriteMessage(websocket.TextMessage, []byte(res))
										last_check_opid = pc.get_last_opid()
									}

								}
							}
						} else if len(expr) == 1 {
							if len(uid_info) > 2 {
								if uid_info[0] == '-' {
									uid := uid_info[1:len(uid_info)]
									delete(pc.count_2_uid, uid)
								}
							}
						}

						//log.Printf("ws[%s]uid_info=%s", pc.ws.RemoteAddr(), uid_info)
					}

				}
			}
		}

		last_opid := pc.get_last_opid()
		if last_check_opid < last_opid {
			res := pc.get_list_of_changes()
			if res != "" {
				err := pc.ws.WriteMessage(websocket.TextMessage, []byte(res))
				if err != nil {
					log.Printf("ERR! NOT SEND: ws[%s] found changes, %s", pc.ws.RemoteAddr(), res)
				} else {
					log.Printf("SEND: ws[%s] found changes, %s", pc.ws.RemoteAddr(), res)
				}
			}
			last_check_opid = last_opid
		}

	}
}

// Receive msg from ws in goroutine
func (pc *ccusConn) receiver() {
	var cc_prepare = make(chan string)
	go pc.preparer(cc_prepare)
	go timer1(cc_prepare)

	log.Printf("ws[%s]:spawn receiver", pc.ws.RemoteAddr())

	for {
		_, bmsg, err := pc.ws.ReadMessage()
		if err != nil {
			break
		}

		msg := string(bmsg)

		cc_prepare <- msg
	}

	log.Printf("ws[%s]:close", pc.ws.RemoteAddr())
	pc.ws.Close()
}

func (pc *ccusConn) sendState() {
	go func() {
		err := pc.ws.WriteMessage(websocket.TextMessage, []byte("msg"))
		if err != nil {
			pc.ws.Close()
		}
	}()
}

func NewCcusConn(ws *websocket.Conn, cc chan updateInfo) *ccusConn {
	pc := &ccusConn{}
	pc.ws = ws
	pc.cc = cc
	go pc.receiver()

	log.Printf("ws[%s]:new connect %s", ws.RemoteAddr(), ws.LocalAddr())

	return pc
}
